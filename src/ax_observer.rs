//! `AXObserver` notification support.

use core::ffi::c_void;
use std::ffi::CString;

use doom_fish_utils::callback_context::CallbackContext;

use crate::ax_error::{AXError, K_AX_ERROR_NOTIFICATION_NOT_REGISTERED, K_AX_ERROR_SUCCESS};
use crate::ax_ui_element::AXUIElement;
use crate::ax_value::AXValue;
use crate::{bridge, internal};

type ObserverCallback = Box<dyn Fn(&AXObserverEvent) + Send + Sync + 'static>;
type ObserverContext = CallbackContext<ObserverCallback>;

#[derive(Clone, Debug)]
/// Notification payload produced by an `AXObserver` callback.
pub struct AXObserverEvent {
    /// Notification name delivered by `AXObserverAddNotification`.
    pub notification: String,
    /// Element argument supplied by the `ApplicationServices` observer callback.
    pub element: AXUIElement,
    /// Optional info payload delivered by `AXObserverCreateWithInfoCallback`.
    pub info: Option<AXValue>,
}

/// Safe owner of an `ApplicationServices` `AXObserverRef`.
pub struct AXObserver {
    observer: *mut c_void,
    registrations: Vec<(AXUIElement, CString)>,
    context: ObserverContext,
}

unsafe impl Send for AXObserver {}

unsafe extern "C" fn observer_callback(
    context: *mut c_void,
    element: *mut c_void,
    notification: *mut c_void,
    info: *mut c_void,
) {
    let notification = unsafe { internal::string_from_handle(notification) };
    let element = (!element.is_null()).then(|| unsafe { AXUIElement::from_raw(element) });
    let info = (!info.is_null()).then(|| unsafe { AXValue::from_raw(info) });
    let (Some(notification), Some(element)) = (notification, element) else {
        return;
    };
    let event = AXObserverEvent {
        notification,
        element,
        info,
    };
    let _ = unsafe {
        ObserverContext::with(context, "AXObserver callback", |callback| {
            callback(&event);
        })
    };
}

impl Drop for AXObserver {
    fn drop(&mut self) {
        self.context.deactivate();
        if self.observer.is_null() {
            return;
        }
        for (element, notification) in &self.registrations {
            // SAFETY: FFI call with valid arguments
            let _ = unsafe {
                bridge::ax_notification::ax_notification_remove(
                    self.observer,
                    element.as_ptr(),
                    notification.as_ptr(),
                )
            };
        }
        unsafe { bridge::ax_observer::ax_observer_release(self.observer) };
        self.observer = core::ptr::null_mut();
    }
}

impl AXObserver {
    #[must_use]
    /// Wraps `AXObserverGetTypeID`.
    pub fn type_id() -> usize {
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_observer::ax_observer_get_type_id() }
    }

    /// Wraps `AXObserverCreate`.
    pub fn new<F>(pid: i32, callback: F) -> Result<Self, AXError>
    where
        F: Fn(&AXObserverEvent) + Send + Sync + 'static,
    {
        Self::new_inner(pid, callback, false)
    }

    /// Wraps `AXObserverCreateWithInfoCallback`.
    pub fn new_with_info<F>(pid: i32, callback: F) -> Result<Self, AXError>
    where
        F: Fn(&AXObserverEvent) + Send + Sync + 'static,
    {
        Self::new_inner(pid, callback, true)
    }

    fn new_inner<F>(pid: i32, callback: F, with_info: bool) -> Result<Self, AXError>
    where
        F: Fn(&AXObserverEvent) + Send + Sync + 'static,
    {
        let context = ObserverContext::new(Box::new(callback));
        let mut observer = core::ptr::null_mut();
        let status = unsafe {
            bridge::ax_observer::ax_observer_create(
                pid,
                with_info,
                Some(observer_callback),
                context.as_ptr(),
                Some(ObserverContext::RETAIN),
                Some(ObserverContext::RELEASE),
                &raw mut observer,
            )
        };
        if status != K_AX_ERROR_SUCCESS {
            return Err(AXError::from_status(status, "AXObserverCreate"));
        }
        Ok(Self {
            observer,
            registrations: Vec::new(),
            context,
        })
    }

    /// Wraps `AXObserverAddNotification`.
    pub fn add_notification(
        &mut self,
        element: &AXUIElement,
        notification: &str,
    ) -> Result<(), AXError> {
        let notification = internal::make_cstring(notification)?;
        // SAFETY: FFI call with valid arguments
        let status = unsafe {
            bridge::ax_notification::ax_notification_add(
                self.observer,
                element.as_ptr(),
                notification.as_ptr(),
            )
        };
        if status != K_AX_ERROR_SUCCESS {
            return Err(AXError::from_status(
                status,
                notification.to_string_lossy().as_ref(),
            ));
        }
        self.registrations.push((element.clone(), notification));
        Ok(())
    }

    pub fn remove_notification(
        &mut self,
        element: &AXUIElement,
        notification: &str,
    ) -> Result<(), AXError> {
        let notification = internal::make_cstring(notification)?;
        let status = unsafe {
            bridge::ax_notification::ax_notification_remove(
                self.observer,
                element.as_ptr(),
                notification.as_ptr(),
            )
        };
        if status == K_AX_ERROR_SUCCESS || status == K_AX_ERROR_NOTIFICATION_NOT_REGISTERED {
            self.registrations.retain(|(registered, name)| {
                *name != notification
                    || !unsafe {
                        bridge::ax_ui_element::ax_ui_element_equal(
                            registered.as_ptr(),
                            element.as_ptr(),
                        )
                    }
            });
        }
        if status == K_AX_ERROR_SUCCESS {
            Ok(())
        } else {
            Err(AXError::from_status(
                status,
                notification.to_string_lossy().as_ref(),
            ))
        }
    }

    /// Schedules the observer run-loop source returned by `AXObserverGetRunLoopSource` on the current loop.
    pub fn schedule_on_current_run_loop(&self) {
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_observer::ax_observer_schedule_on_current_run_loop(self.observer) };
    }

    /// Unschedules the observer run-loop source returned by `AXObserverGetRunLoopSource` from the current loop.
    pub fn unschedule_from_run_loop(&self) {
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_observer::ax_observer_unschedule_from_run_loop(self.observer) };
    }
}

/// Wraps `CFRunLoopRun` while waiting for Accessibility notifications.
pub fn run_current_run_loop() {
    // SAFETY: FFI boundary with properly validated inputs
    unsafe { bridge::ax_observer::ax_run_current_run_loop() };
}

/// Wraps `CFRunLoopStop` for the current run loop.
pub fn stop_current_run_loop() {
    // SAFETY: FFI boundary with properly validated inputs
    unsafe { bridge::ax_observer::ax_stop_current_run_loop() };
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use super::{observer_callback, ObserverContext};
    use crate::bridge;
    use crate::AXUIElement;

    fn notification_handle(name: &core::ffi::CStr) -> *mut core::ffi::c_void {
        let handle = unsafe { bridge::ax_value::ax_value_create_string(name.as_ptr()) };
        assert!(!handle.is_null());
        handle
    }

    fn element_handle() -> *mut core::ffi::c_void {
        let system = AXUIElement::system_wide().expect("system-wide element");
        let handle = unsafe { bridge::ax_ui_element::ax_ui_element_retain(system.as_ptr()) };
        assert!(!handle.is_null());
        handle
    }

    #[test]
    fn trampoline_delivers_until_the_context_is_deactivated() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let context = ObserverContext::new(Box::new(move |event| {
            sink.lock().unwrap().push(event.notification.clone());
        }));
        let foreign = context.retained_ptr();

        unsafe {
            observer_callback(
                foreign,
                element_handle(),
                notification_handle(c"AXTitleChanged"),
                core::ptr::null_mut(),
            );
        }
        assert_eq!(*seen.lock().unwrap(), vec!["AXTitleChanged".to_string()]);

        context.deactivate();
        unsafe {
            observer_callback(
                foreign,
                element_handle(),
                notification_handle(c"AXMoved"),
                core::ptr::null_mut(),
            );
        }
        assert_eq!(seen.lock().unwrap().len(), 1);

        drop(context);
        assert_eq!(Arc::strong_count(&seen), 2);
        unsafe { (ObserverContext::RELEASE)(foreign) };
        assert_eq!(Arc::strong_count(&seen), 1);
    }

    #[test]
    fn trampoline_skips_incomplete_events_and_contains_panics() {
        let calls = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&calls);
        let context = ObserverContext::new(Box::new(move |_| {
            counter.fetch_add(1, Ordering::SeqCst);
            panic!("observer callback panic");
        }));

        unsafe {
            observer_callback(
                context.as_ptr(),
                core::ptr::null_mut(),
                notification_handle(c"AXMoved"),
                core::ptr::null_mut(),
            );
            observer_callback(
                context.as_ptr(),
                element_handle(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
        }
        assert_eq!(calls.load(Ordering::SeqCst), 0);

        unsafe {
            observer_callback(
                context.as_ptr(),
                element_handle(),
                notification_handle(c"AXMoved"),
                core::ptr::null_mut(),
            );
        }
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
