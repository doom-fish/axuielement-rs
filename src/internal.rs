use core::ffi::{c_char, c_void};
use std::ffi::CString;

use crate::ax_error::AXError;
use crate::bridge::ax_value::{ax_string_copy_utf8, ax_string_len, ax_string_release};

pub(crate) fn make_cstring(value: &str) -> Result<CString, AXError> {
    CString::new(value).map_err(|err| AXError::IllegalArgument(format!("CString: {err}")))
}

pub(crate) fn make_cstring_vec(
    values: &[&str],
) -> Result<(Vec<CString>, Vec<*const c_char>), AXError> {
    let owned = values
        .iter()
        .map(|value| make_cstring(value))
        .collect::<Result<Vec<_>, _>>()?;
    let raw = owned.iter().map(|value| value.as_ptr()).collect();
    Ok((owned, raw))
}

pub(crate) unsafe fn string_from_handle(handle: *mut c_void) -> Option<String> {
    if handle.is_null() {
        return None;
    }
    let len = unsafe { ax_string_len(handle) };
    let mut buffer = vec![0_u8; len.saturating_add(1)];
    let ok =
        unsafe { ax_string_copy_utf8(handle, buffer.as_mut_ptr().cast::<c_char>(), buffer.len()) };
    unsafe { ax_string_release(handle) };
    if !ok {
        return None;
    }
    if let Some(end) = buffer.iter().position(|byte| *byte == 0) {
        buffer.truncate(end);
    }
    String::from_utf8(buffer).ok()
}
