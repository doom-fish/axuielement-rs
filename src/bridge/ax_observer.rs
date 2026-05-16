use core::ffi::c_void;

use super::{AXStatus, Handle};

pub type RustObserverCallback = unsafe extern "C" fn(
    observer: Handle,
    element: Handle,
    notification: Handle,
    refcon: *mut c_void,
);

pub type RustObserverInfoCallback = unsafe extern "C" fn(
    observer: Handle,
    element: Handle,
    notification: Handle,
    info: Handle,
    refcon: *mut c_void,
);

extern "C" {
    pub fn ax_observer_get_type_id() -> usize;
    pub fn ax_observer_create(
        pid: i32,
        callback: Option<RustObserverCallback>,
        out_observer: *mut Handle,
    ) -> AXStatus;
    pub fn ax_observer_create_with_info(
        pid: i32,
        callback: Option<RustObserverInfoCallback>,
        out_observer: *mut Handle,
    ) -> AXStatus;
    pub fn ax_observer_release(handle: Handle);
    pub fn ax_observer_schedule_on_current_run_loop(handle: Handle);
    pub fn ax_observer_unschedule_from_run_loop(handle: Handle);
    pub fn ax_run_current_run_loop();
    pub fn ax_stop_current_run_loop();
}
