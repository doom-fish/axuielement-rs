use core::ffi::c_char;

use super::Handle;
use crate::ax_value::{AXPoint, AXRange, AXRect, AXSize};

extern "C" {
    pub fn ax_string_release(handle: Handle);
    pub fn ax_string_len(handle: Handle) -> usize;
    pub fn ax_string_copy_utf8(handle: Handle, buffer: *mut c_char, capacity: usize) -> bool;

    pub fn ax_value_get_type_id() -> usize;
    pub fn ax_value_retain(handle: Handle) -> Handle;
    pub fn ax_value_release(handle: Handle);
    pub fn ax_value_kind(handle: Handle) -> u32;
    pub fn ax_value_copy_string(handle: Handle) -> Handle;
    pub fn ax_value_get_bool(handle: Handle, out_value: *mut bool) -> bool;
    pub fn ax_value_get_i64(handle: Handle, out_value: *mut i64) -> bool;
    pub fn ax_value_get_f64(handle: Handle, out_value: *mut f64) -> bool;
    pub fn ax_value_get_point(handle: Handle, out_value: *mut AXPoint) -> bool;
    pub fn ax_value_get_size(handle: Handle, out_value: *mut AXSize) -> bool;
    pub fn ax_value_get_rect(handle: Handle, out_value: *mut AXRect) -> bool;
    pub fn ax_value_get_range(handle: Handle, out_value: *mut AXRange) -> bool;
    pub fn ax_value_get_error_code(handle: Handle, out_value: *mut i32) -> bool;
    pub fn ax_value_array_len(handle: Handle) -> usize;
    pub fn ax_value_array_get(handle: Handle, index: usize) -> Handle;
    pub fn ax_value_dictionary_len(handle: Handle) -> usize;
    pub fn ax_value_dictionary_key_at(handle: Handle, index: usize) -> Handle;
    pub fn ax_value_dictionary_value_at(handle: Handle, index: usize) -> Handle;
    pub fn ax_value_data_len(handle: Handle) -> usize;
    pub fn ax_value_copy_data_bytes(handle: Handle, buffer: *mut u8, capacity: usize) -> bool;

    pub fn ax_value_create_null() -> Handle;
    pub fn ax_value_create_string(value: *const c_char) -> Handle;
    pub fn ax_value_create_bool(value: bool) -> Handle;
    pub fn ax_value_create_i64(value: i64) -> Handle;
    pub fn ax_value_create_f64(value: f64) -> Handle;
    pub fn ax_value_create_point(value: AXPoint) -> Handle;
    pub fn ax_value_create_size(value: AXSize) -> Handle;
    pub fn ax_value_create_rect(value: AXRect) -> Handle;
    pub fn ax_value_create_range(value: AXRange) -> Handle;
    pub fn ax_value_create_error_code(code: i32) -> Handle;
    pub fn ax_value_create_element(handle: Handle) -> Handle;
    pub fn ax_value_create_text_marker(handle: Handle) -> Handle;
    pub fn ax_value_create_text_marker_range(handle: Handle) -> Handle;
    pub fn ax_value_create_data(bytes: *const u8, length: usize) -> Handle;
    pub fn ax_value_create_array(values: *const Handle, count: usize) -> Handle;
    pub fn ax_value_create_dictionary(
        keys: *const *const c_char,
        values: *const Handle,
        count: usize,
    ) -> Handle;
}
