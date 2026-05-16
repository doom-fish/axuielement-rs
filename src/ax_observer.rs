//! `AXObserver` notification support.

use core::ffi::c_void;
use std::sync::{Arc, Mutex};

use crate::ax_error::{AXError, K_AX_ERROR_SUCCESS};
use crate::ax_ui_element::AXUIElement;
use crate::ax_value::AXValue;
use crate::{bridge, internal};

type CallbackFn = Box<dyn Fn(&AXObserverEvent) + Send + Sync + 'static>;

struct ObserverState {
    callback: Mutex<CallbackFn>,
}

#[derive(Clone, Debug)]
pub struct AXObserverEvent {
    pub notification: String,
    pub element: AXUIElement,
    pub info: Option<AXValue>,
}

pub struct AXObserver {
    observer: *mut c_void,
    registrations: Vec<(AXUIElement, std::ffi::CString)>,
    state: Arc<ObserverState>,
}

unsafe impl Send for AXObserver {}

unsafe extern "C" fn observer_callback(
    observer: *mut c_void,
    element: *mut c_void,
    notification: *mut c_void,
    refcon: *mut c_void,
) {
    let notification_text = unsafe { internal::string_from_handle(notification) };
    let event_element = if element.is_null() {
        None
    } else {
        Some(unsafe { AXUIElement::from_raw(element) })
    };
    if !observer.is_null() {
        unsafe { bridge::ax_observer::ax_observer_release(observer) };
    }
    let Some(notification) = notification_text else {
        return;
    };
    let Some(element) = event_element else {
        return;
    };
    if refcon.is_null() {
        return;
    }
    let state = unsafe { &*refcon.cast::<ObserverState>() };
    let Ok(callback) = state.callback.lock() else {
        return;
    };
    callback(&AXObserverEvent {
        notification,
        element,
        info: None,
    });
}

unsafe extern "C" fn observer_info_callback(
    observer: *mut c_void,
    element: *mut c_void,
    notification: *mut c_void,
    info: *mut c_void,
    refcon: *mut c_void,
) {
    let notification_text = unsafe { internal::string_from_handle(notification) };
    let event_element = if element.is_null() {
        None
    } else {
        Some(unsafe { AXUIElement::from_raw(element) })
    };
    let event_info = (!info.is_null()).then(|| unsafe { AXValue::from_raw(info) });
    if !observer.is_null() {
        unsafe { bridge::ax_observer::ax_observer_release(observer) };
    }
    let Some(notification) = notification_text else {
        return;
    };
    let Some(element) = event_element else {
        return;
    };
    if refcon.is_null() {
        return;
    }
    let state = unsafe { &*refcon.cast::<ObserverState>() };
    let Ok(callback) = state.callback.lock() else {
        return;
    };
    callback(&AXObserverEvent {
        notification,
        element,
        info: event_info,
    });
}

impl Drop for AXObserver {
    fn drop(&mut self) {
        if self.observer.is_null() {
            return;
        }
        for (element, notification) in &self.registrations {
            let _ = unsafe {
                bridge::ax_notification::ax_notification_remove(
                    self.observer,
                    element.as_ptr(),
                    notification.as_ptr(),
                )
            };
        }
        unsafe {
            bridge::ax_observer::ax_observer_unschedule_from_run_loop(self.observer);
            bridge::ax_observer::ax_observer_release(self.observer);
        }
        self.observer = core::ptr::null_mut();
    }
}

impl AXObserver {
    #[must_use]
    pub fn type_id() -> usize {
        unsafe { bridge::ax_observer::ax_observer_get_type_id() }
    }

    pub fn new<F>(pid: i32, callback: F) -> Result<Self, AXError>
    where
        F: Fn(&AXObserverEvent) + Send + Sync + 'static,
    {
        Self::new_inner(pid, callback, false)
    }

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
        let state = Arc::new(ObserverState {
            callback: Mutex::new(Box::new(callback)),
        });
        let mut observer = core::ptr::null_mut();
        let status = unsafe {
            if with_info {
                bridge::ax_observer::ax_observer_create_with_info(
                    pid,
                    Some(observer_info_callback),
                    &mut observer,
                )
            } else {
                bridge::ax_observer::ax_observer_create(pid, Some(observer_callback), &mut observer)
            }
        };
        if status != K_AX_ERROR_SUCCESS {
            return Err(AXError::from_status(status, "AXObserverCreate"));
        }
        Ok(Self {
            observer,
            registrations: Vec::new(),
            state,
        })
    }

    pub fn add_notification(
        &mut self,
        element: &AXUIElement,
        notification: &str,
    ) -> Result<(), AXError> {
        let notification = internal::make_cstring(notification)?;
        let refcon = Arc::as_ptr(&self.state)
            .cast::<ObserverState>()
            .cast_mut()
            .cast::<c_void>();
        let status = unsafe {
            bridge::ax_notification::ax_notification_add(
                self.observer,
                element.as_ptr(),
                notification.as_ptr(),
                refcon,
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

    pub fn schedule_on_current_run_loop(&self) {
        unsafe { bridge::ax_observer::ax_observer_schedule_on_current_run_loop(self.observer) };
    }

    pub fn unschedule_from_run_loop(&self) {
        unsafe { bridge::ax_observer::ax_observer_unschedule_from_run_loop(self.observer) };
    }
}

pub fn run_current_run_loop() {
    unsafe { bridge::ax_observer::ax_run_current_run_loop() };
}

pub fn stop_current_run_loop() {
    unsafe { bridge::ax_observer::ax_stop_current_run_loop() };
}
