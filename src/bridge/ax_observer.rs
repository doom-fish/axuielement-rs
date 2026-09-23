use core::ffi::c_void;

use super::{AXStatus, Handle};

pub type RustObserverCallback =
    unsafe extern "C" fn(context: *mut c_void, element: Handle, notification: Handle, info: Handle);

pub type RustObserverContextHook = unsafe extern "C" fn(context: *mut c_void);

extern "C" {
    pub fn ax_observer_get_type_id() -> usize;
    pub fn ax_observer_create(
        pid: i32,
        with_info: bool,
        callback: Option<RustObserverCallback>,
        context: *mut c_void,
        retain_context: Option<RustObserverContextHook>,
        release_context: Option<RustObserverContextHook>,
        out_observer: *mut Handle,
    ) -> AXStatus;
    pub fn ax_observer_release(handle: Handle);
    pub fn ax_observer_schedule_on_current_run_loop(handle: Handle);
    pub fn ax_observer_unschedule_from_run_loop(handle: Handle);
    pub fn ax_run_current_run_loop();
    pub fn ax_stop_current_run_loop();
}
