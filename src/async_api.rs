//! Executor-agnostic async streams for Accessibility notifications.
//!
//! Enabled with the `async` Cargo feature.
//!
//! [`AXNotificationStream`] wraps the `AXObserver` callback model in a
//! runtime-agnostic bounded async stream backed by
//! [`doom_fish_utils::stream::BoundedAsyncStream`]. The stream owns a dedicated
//! run-loop thread; dropping it stops that thread, unschedules the observer, and
//! closes the stream.
//!
//! # Example
//!
//! ```no_run
//! use axuielement::async_api::AXNotificationStream;
//! use axuielement::ax_notification::AX_MAIN_WINDOW_CHANGED_NOTIFICATION;
//! use axuielement::AXUIElement;
//!
//! # async fn run() -> Result<(), axuielement::AXError> {
//! let pid = i32::try_from(std::process::id()).expect("current pid fits in i32");
//! let app = AXUIElement::from_pid(pid).expect("AX application handle");
//! let stream = AXNotificationStream::subscribe_many(
//!     &app,
//!     &[AX_MAIN_WINDOW_CHANGED_NOTIFICATION],
//!     8,
//! )?;
//!
//! while let Some(event) = stream.next().await {
//!     println!("notification={} element={:?}", event.notification, event.element);
//! }
//! # Ok(())
//! # }
//! ```

#![cfg(feature = "async")]

use core::ffi::c_void;
use core::fmt;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use doom_fish_utils::stream::{BoundedAsyncStream, NextItem};

use crate::{AXError, AXObserver, AXObserverEvent, AXUIElement};

unsafe extern "C" {
    fn CFRunLoopGetCurrent() -> *mut c_void;
    fn CFRunLoopRun();
    fn CFRunLoopStop(rl: *mut c_void);
    fn CFRunLoopWakeUp(rl: *mut c_void);
}

struct ObserverThreadHandle {
    run_loop: *mut c_void,
    join: Option<JoinHandle<()>>,
}

unsafe impl Send for ObserverThreadHandle {}
unsafe impl Sync for ObserverThreadHandle {}

impl Drop for ObserverThreadHandle {
    fn drop(&mut self) {
        if !self.run_loop.is_null() {
            unsafe {
                CFRunLoopStop(self.run_loop);
                CFRunLoopWakeUp(self.run_loop);
            }
        }
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

impl fmt::Debug for ObserverThreadHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ObserverThreadHandle")
            .field("run_loop", &self.run_loop)
            .field("thread_running", &self.join.is_some())
            .finish_non_exhaustive()
    }
}

/// Async stream of [`AXObserverEvent`] values produced by Accessibility notifications.
#[derive(Debug)]
pub struct AXNotificationStream {
    inner: BoundedAsyncStream<AXObserverEvent>,
    _handle: ObserverThreadHandle,
}

impl AXNotificationStream {
    /// Subscribe to one or more notifications for `element`.
    ///
    /// The stream stays open until it is dropped.
    pub fn subscribe_many(
        element: &AXUIElement,
        notifications: &[&str],
        capacity: usize,
    ) -> Result<Self, AXError> {
        Self::subscribe_inner(element, notifications, capacity, false)
    }

    /// Subscribe to one or more notifications for `element`, preserving the optional info payload.
    pub fn subscribe_many_with_info(
        element: &AXUIElement,
        notifications: &[&str],
        capacity: usize,
    ) -> Result<Self, AXError> {
        Self::subscribe_inner(element, notifications, capacity, true)
    }

    fn subscribe_inner(
        element: &AXUIElement,
        notifications: &[&str],
        capacity: usize,
        with_info: bool,
    ) -> Result<Self, AXError> {
        if capacity == 0 {
            return Err(AXError::IllegalArgument(
                "async stream capacity must be > 0".into(),
            ));
        }

        let pid = element.pid()?;
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let mut sender = Some(sender);
        let mut observer = if with_info {
            let sender = sender.take().expect("stream sender available");
            AXObserver::new_with_info(pid, move |event| sender.push(event.clone()))?
        } else {
            let sender = sender.take().expect("stream sender available");
            AXObserver::new(pid, move |event| sender.push(event.clone()))?
        };

        for notification in notifications {
            observer.add_notification(element, notification)?;
        }

        let (run_loop_tx, run_loop_rx) = mpsc::sync_channel(1);
        let join = thread::spawn(move || {
            observer.schedule_on_current_run_loop();
            let run_loop = unsafe { CFRunLoopGetCurrent() };
            let _ = run_loop_tx.send(run_loop as usize);
            unsafe { CFRunLoopRun() };
            observer.unschedule_from_run_loop();
        });

        let run_loop = match run_loop_rx.recv() {
            Ok(run_loop) if run_loop != 0 => run_loop as *mut c_void,
            _ => {
                let _ = join.join();
                return Err(AXError::CannotComplete);
            }
        };

        Ok(Self {
            inner: stream,
            _handle: ObserverThreadHandle {
                run_loop,
                join: Some(join),
            },
        })
    }

    /// Await the next notification event.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, AXObserverEvent> {
        self.inner.next()
    }

    /// Return the next buffered event without waiting.
    #[must_use]
    pub fn try_next(&self) -> Option<AXObserverEvent> {
        self.inner.try_next()
    }

    /// Return the number of buffered events.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }

    /// Return `true` once the stream has been dropped and fully drained.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}
