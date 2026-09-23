use core::ffi::c_char;

use super::{AXStatus, Handle};

extern "C" {
    pub fn ax_notification_add(
        observer_handle: Handle,
        element_handle: Handle,
        notification: *const c_char,
    ) -> AXStatus;
    pub fn ax_notification_remove(
        observer_handle: Handle,
        element_handle: Handle,
        notification: *const c_char,
    ) -> AXStatus;
}
