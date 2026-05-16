use core::ffi::c_char;

use super::{AXStatus, Handle};

extern "C" {
    pub fn ax_attribute_copy_names(handle: Handle, out_value: *mut Handle) -> AXStatus;
    pub fn ax_attribute_is_settable(
        handle: Handle,
        attribute: *const c_char,
        out_value: *mut bool,
    ) -> AXStatus;
    pub fn ax_attribute_get_value_count(
        handle: Handle,
        attribute: *const c_char,
        out_count: *mut isize,
    ) -> AXStatus;
    pub fn ax_attribute_copy_value(
        handle: Handle,
        attribute: *const c_char,
        out_value: *mut Handle,
    ) -> AXStatus;
    pub fn ax_attribute_copy_values(
        handle: Handle,
        attribute: *const c_char,
        index: isize,
        max_values: isize,
        out_value: *mut Handle,
    ) -> AXStatus;
    pub fn ax_attribute_set_value(
        handle: Handle,
        attribute: *const c_char,
        value_handle: Handle,
    ) -> AXStatus;
    pub fn ax_attribute_copy_multiple_values(
        handle: Handle,
        attributes: *const *const c_char,
        count: usize,
        options: u32,
        out_value: *mut Handle,
    ) -> AXStatus;
    pub fn ax_attribute_copy_parameterized_names(
        handle: Handle,
        out_value: *mut Handle,
    ) -> AXStatus;
    pub fn ax_attribute_copy_parameterized_value(
        handle: Handle,
        attribute: *const c_char,
        parameter_handle: Handle,
        out_value: *mut Handle,
    ) -> AXStatus;
}
