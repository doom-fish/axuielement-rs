//! `AXObserver` — subscribe to UI notifications from another app.

use core::ffi::c_void;
use core::ptr;
use std::ffi::CString;
use std::sync::{Arc, Mutex};

use crate::element::AXElement;
use crate::error::AXError;
use crate::ffi;

type CallbackFn = Box<dyn Fn(&AXObserverEvent) + Send + Sync + 'static>;

struct ObserverState {
    callback: Mutex<CallbackFn>,
}

/// One notification delivered by an [`AXObserver`].
#[derive(Debug, Clone)]
pub struct AXObserverEvent {
    pub notification: String,
}

/// RAII guard for an `AXObserver` registration. Drops the observer +
/// removes it from the current thread's run loop.
pub struct AXObserver {
    observer: ffi::AXObserverRef,
    source: *mut c_void,
    elements: Vec<*mut c_void>,
    notifications: Vec<CString>,
    state: Arc<ObserverState>,
}

unsafe impl Send for AXObserver {}

impl Drop for AXObserver {
    fn drop(&mut self) {
        if !self.observer.is_null() {
            unsafe {
                for (elem, notif) in self.elements.iter().zip(self.notifications.iter()) {
                    let cf = ffi::CFStringCreateWithCString(
                        ffi::kCFAllocatorDefault,
                        notif.as_ptr(),
                        ffi::kCFStringEncodingUTF8,
                    );
                    if !cf.is_null() {
                        ffi::AXObserverRemoveNotification(self.observer, *elem, cf);
                        ffi::CFRelease(cf);
                    }
                }
                if !self.source.is_null() {
                    ffi::CFRunLoopRemoveSource(
                        ffi::CFRunLoopGetCurrent(),
                        self.source,
                        ffi::kCFRunLoopDefaultMode,
                    );
                }
                ffi::CFRelease(self.observer.cast_const());
            }
            self.observer = ptr::null_mut();
        }
    }
}

unsafe extern "C" fn observer_trampoline(
    _observer: ffi::AXObserverRef,
    _element: ffi::AXUIElementRef,
    notification: ffi::CFStringRef,
    refcon: *mut c_void,
) {
    if refcon.is_null() {
        return;
    }
    let state = unsafe { &*refcon.cast::<ObserverState>() };
    let len = unsafe { ffi::CFStringGetLength(notification) };
    let cap = (len * 4) + 1;
    let mut buf = vec![0u8; usize::try_from(cap).unwrap_or(0)];
    let ok = unsafe {
        ffi::CFStringGetCString(
            notification,
            buf.as_mut_ptr().cast::<core::ffi::c_char>(),
            cap,
            ffi::kCFStringEncodingUTF8,
        )
    };
    if !ok {
        return;
    }
    if let Some(end) = buf.iter().position(|&b| b == 0) {
        buf.truncate(end);
    }
    let Ok(notif) = String::from_utf8(buf) else {
        return;
    };
    let event = AXObserverEvent { notification: notif };
    let Ok(cb) = state.callback.lock() else {
        return;
    };
    cb(&event);
}

impl AXObserver {
    /// Create an observer attached to the PID's process. Pass a closure
    /// that fires on every notification.
    ///
    /// # Errors
    ///
    /// Returns [`AXError`] from `AXObserverCreate` on failure.
    pub fn new<F>(pid: i32, callback: F) -> Result<Self, AXError>
    where
        F: Fn(&AXObserverEvent) + Send + Sync + 'static,
    {
        let state = Arc::new(ObserverState {
            callback: Mutex::new(Box::new(callback)),
        });
        let mut observer: ffi::AXObserverRef = ptr::null_mut();
        let status =
            unsafe { ffi::AXObserverCreate(pid, observer_trampoline, &mut observer) };
        if status != ffi::kAXErrorSuccess {
            return Err(AXError::from_status(status, "AXObserverCreate"));
        }
        let source = unsafe { ffi::AXObserverGetRunLoopSource(observer) };
        Ok(Self {
            observer,
            source,
            elements: Vec::new(),
            notifications: Vec::new(),
            state,
        })
    }

    /// Subscribe to `notification` on `element`. Common notification
    /// names: `kAXFocusedWindowChangedNotification`,
    /// `kAXValueChangedNotification`, `kAXTitleChangedNotification`.
    ///
    /// # Errors
    ///
    /// Returns [`AXError`] from `AXObserverAddNotification` on failure.
    pub fn add_notification(
        &mut self,
        element: &AXElement,
        notification: &str,
    ) -> Result<(), AXError> {
        let notif_c = CString::new(notification)
            .map_err(|e| AXError::IllegalArgument(format!("CString: {e}")))?;
        let notif_cf = unsafe {
            ffi::CFStringCreateWithCString(
                ffi::kCFAllocatorDefault,
                notif_c.as_ptr(),
                ffi::kCFStringEncodingUTF8,
            )
        };
        if notif_cf.is_null() {
            return Err(AXError::IllegalArgument(format!(
                "CFString failed for {notification:?}"
            )));
        }
        let refcon = Arc::as_ptr(&self.state).cast::<c_void>().cast_mut();
        let status = unsafe {
            ffi::AXObserverAddNotification(self.observer, element.as_ptr(), notif_cf, refcon)
        };
        unsafe { ffi::CFRelease(notif_cf) };
        if status != ffi::kAXErrorSuccess {
            return Err(AXError::from_status(status, notification));
        }
        self.elements.push(element.as_ptr());
        self.notifications.push(notif_c);
        Ok(())
    }

    /// Attach the observer to the calling thread's run loop. Callbacks
    /// won't fire until you drive that run loop (`CFRunLoopRun`, or your
    /// app's existing event loop).
    pub fn schedule_on_current_run_loop(&self) {
        if self.source.is_null() {
            return;
        }
        unsafe {
            ffi::CFRunLoopAddSource(
                ffi::CFRunLoopGetCurrent(),
                self.source,
                ffi::kCFRunLoopDefaultMode,
            );
        }
    }
}

/// Run the current thread's run loop forever (or until
/// [`stop_current_run_loop`] is called).
pub fn run_current_run_loop() {
    unsafe { ffi::CFRunLoopRun() };
}

/// Stop a running run loop. Call from another thread.
pub fn stop_current_run_loop() {
    unsafe { ffi::CFRunLoopStop(ffi::CFRunLoopGetCurrent()) };
}
