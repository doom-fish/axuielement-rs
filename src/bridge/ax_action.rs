use core::ffi::c_char;

use super::{AXStatus, Handle};

extern "C" {
    pub fn ax_action_copy_names(handle: Handle, out_value: *mut Handle) -> AXStatus;
    pub fn ax_action_copy_description(
        handle: Handle,
        action: *const c_char,
        out_value: *mut Handle,
    ) -> AXStatus;
    pub fn ax_action_perform(handle: Handle, action: *const c_char) -> AXStatus;
}
