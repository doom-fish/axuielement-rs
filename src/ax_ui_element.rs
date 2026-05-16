//! Safe wrapper for `AXUIElement`.

use core::ffi::c_void;
use core::fmt;

use crate::ax_error::{AXError, K_AX_ERROR_NO_VALUE, K_AX_ERROR_SUCCESS};
use crate::ax_text_marker::{AXTextMarker, AXTextMarkerRange};
use crate::ax_value::{AXPoint, AXRange, AXRect, AXSize, AXValue};
use crate::{bridge, internal};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AXCopyMultipleAttributeOptions(u32);

impl AXCopyMultipleAttributeOptions {
    pub const NONE: Self = Self(0);
    pub const STOP_ON_ERROR: Self = Self(1);

    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl core::ops::BitOr for AXCopyMultipleAttributeOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[repr(transparent)]
pub struct AXUIElement {
    raw: *mut c_void,
}

pub type AXElement = AXUIElement;

unsafe impl Send for AXUIElement {}
unsafe impl Sync for AXUIElement {}

impl Clone for AXUIElement {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::ax_ui_element::ax_ui_element_retain(self.raw) };
        unsafe { Self::from_raw(raw) }
    }
}

impl Drop for AXUIElement {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { bridge::ax_ui_element::ax_ui_element_release(self.raw) };
            self.raw = core::ptr::null_mut();
        }
    }
}

impl fmt::Debug for AXUIElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AXUIElement")
            .field("pid", &self.pid())
            .finish()
    }
}

impl AXUIElement {
    #[must_use]
    pub fn type_id() -> usize {
        unsafe { bridge::ax_ui_element::ax_ui_element_get_type_id() }
    }

    #[must_use]
    pub fn from_pid(pid: i32) -> Option<Self> {
        let raw = unsafe { bridge::ax_ui_element::ax_ui_element_create_application(pid) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn system_wide() -> Option<Self> {
        let raw = unsafe { bridge::system_wide::ax_system_wide_create() };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    pub fn pid(&self) -> Result<i32, AXError> {
        let mut pid = 0_i32;
        let status = unsafe { bridge::ax_ui_element::ax_ui_element_get_pid(self.raw, &mut pid) };
        if status == K_AX_ERROR_SUCCESS {
            Ok(pid)
        } else {
            Err(AXError::from_status(status, "AXUIElementGetPid"))
        }
    }

    pub fn set_timeout(&self, timeout_seconds: f32) -> Result<(), AXError> {
        let status = unsafe {
            bridge::ax_ui_element::ax_ui_element_set_messaging_timeout(self.raw, timeout_seconds)
        };
        if status == K_AX_ERROR_SUCCESS {
            Ok(())
        } else {
            Err(AXError::from_status(status, "set_timeout"))
        }
    }

    pub fn attribute_names(&self) -> Result<Vec<String>, AXError> {
        let mut raw = core::ptr::null_mut();
        let status = unsafe { bridge::ax_attribute::ax_attribute_copy_names(self.raw, &mut raw) };
        if status != K_AX_ERROR_SUCCESS {
            return Err(AXError::from_status(status, "attribute_names"));
        }
        Ok(raw_to_string_vec(raw))
    }

    pub fn is_attribute_settable(&self, name: &str) -> Result<bool, AXError> {
        let name = internal::make_cstring(name)?;
        let mut settable = false;
        let status = unsafe {
            bridge::ax_attribute::ax_attribute_is_settable(self.raw, name.as_ptr(), &mut settable)
        };
        if status == K_AX_ERROR_SUCCESS {
            Ok(settable)
        } else if status == K_AX_ERROR_NO_VALUE {
            Ok(false)
        } else {
            Err(AXError::from_status(
                status,
                name.to_string_lossy().as_ref(),
            ))
        }
    }

    pub fn attribute_value_count(&self, name: &str) -> Result<usize, AXError> {
        let name = internal::make_cstring(name)?;
        let mut count = 0_isize;
        let status = unsafe {
            bridge::ax_attribute::ax_attribute_get_value_count(self.raw, name.as_ptr(), &mut count)
        };
        if status == K_AX_ERROR_SUCCESS {
            usize::try_from(count).map_err(|_| {
                AXError::IllegalArgument(format!("count overflow for {}", name.to_string_lossy()))
            })
        } else if status == K_AX_ERROR_NO_VALUE {
            Ok(0)
        } else {
            Err(AXError::from_status(
                status,
                name.to_string_lossy().as_ref(),
            ))
        }
    }

    pub fn attribute(&self, name: &str) -> Result<Option<AXValue>, AXError> {
        let name = internal::make_cstring(name)?;
        let mut raw = core::ptr::null_mut();
        let status = unsafe {
            bridge::ax_attribute::ax_attribute_copy_value(self.raw, name.as_ptr(), &mut raw)
        };
        if status == K_AX_ERROR_SUCCESS {
            Ok((!raw.is_null()).then(|| unsafe { AXValue::from_raw(raw) }))
        } else if status == K_AX_ERROR_NO_VALUE {
            Ok(None)
        } else {
            Err(AXError::from_status(
                status,
                name.to_string_lossy().as_ref(),
            ))
        }
    }

    pub fn string_attribute(&self, name: &str) -> Result<Option<String>, AXError> {
        Ok(self.attribute(name)?.and_then(|value| value.as_string()))
    }

    pub fn bool_attribute(&self, name: &str) -> Result<Option<bool>, AXError> {
        Ok(self.attribute(name)?.and_then(|value| value.as_bool()))
    }

    pub fn i64_attribute(&self, name: &str) -> Result<Option<i64>, AXError> {
        Ok(self.attribute(name)?.and_then(|value| value.as_i64()))
    }

    pub fn f64_attribute(&self, name: &str) -> Result<Option<f64>, AXError> {
        Ok(self.attribute(name)?.and_then(|value| value.as_f64()))
    }

    pub fn point_attribute(&self, name: &str) -> Result<Option<AXPoint>, AXError> {
        Ok(self.attribute(name)?.and_then(|value| value.as_point()))
    }

    pub fn size_attribute(&self, name: &str) -> Result<Option<AXSize>, AXError> {
        Ok(self.attribute(name)?.and_then(|value| value.as_size()))
    }

    pub fn rect_attribute(&self, name: &str) -> Result<Option<AXRect>, AXError> {
        Ok(self.attribute(name)?.and_then(|value| value.as_rect()))
    }

    pub fn range_attribute(&self, name: &str) -> Result<Option<AXRange>, AXError> {
        Ok(self.attribute(name)?.and_then(|value| value.as_range()))
    }

    pub fn element_attribute(&self, name: &str) -> Result<Option<Self>, AXError> {
        Ok(self.attribute(name)?.and_then(|value| value.as_element()))
    }

    pub fn text_marker_attribute(&self, name: &str) -> Result<Option<AXTextMarker>, AXError> {
        Ok(self
            .attribute(name)?
            .and_then(|value| value.as_text_marker()))
    }

    pub fn text_marker_range_attribute(
        &self,
        name: &str,
    ) -> Result<Option<AXTextMarkerRange>, AXError> {
        Ok(self
            .attribute(name)?
            .and_then(|value| value.as_text_marker_range()))
    }

    pub fn value_array_attribute_range(
        &self,
        name: &str,
        index: usize,
        max_values: usize,
    ) -> Result<Vec<AXValue>, AXError> {
        let name = internal::make_cstring(name)?;
        let index = isize::try_from(index).map_err(|_| {
            AXError::IllegalArgument(format!("index overflow for {}", name.to_string_lossy()))
        })?;
        let max_values = isize::try_from(max_values).map_err(|_| {
            AXError::IllegalArgument(format!(
                "max_values overflow for {}",
                name.to_string_lossy()
            ))
        })?;
        let mut raw = core::ptr::null_mut();
        let status = unsafe {
            bridge::ax_attribute::ax_attribute_copy_values(
                self.raw,
                name.as_ptr(),
                index,
                max_values,
                &mut raw,
            )
        };
        if status == K_AX_ERROR_SUCCESS {
            Ok(raw_to_value_vec(raw))
        } else if status == K_AX_ERROR_NO_VALUE {
            Ok(Vec::new())
        } else {
            Err(AXError::from_status(
                status,
                name.to_string_lossy().as_ref(),
            ))
        }
    }

    pub fn element_array_attribute_range(
        &self,
        name: &str,
        index: usize,
        max_values: usize,
    ) -> Result<Vec<Self>, AXError> {
        Ok(self
            .value_array_attribute_range(name, index, max_values)?
            .into_iter()
            .filter_map(|value| value.as_element())
            .collect())
    }

    pub fn element_array_attribute(&self, name: &str) -> Result<Vec<Self>, AXError> {
        let count = self.attribute_value_count(name)?;
        if count == 0 {
            return Ok(Vec::new());
        }
        self.element_array_attribute_range(name, 0, count)
    }

    pub fn children(&self) -> Result<Vec<Self>, AXError> {
        self.element_array_attribute("AXChildren")
    }

    pub fn set_attribute(&self, name: &str, value: &AXValue) -> Result<(), AXError> {
        let name = internal::make_cstring(name)?;
        let status = unsafe {
            bridge::ax_attribute::ax_attribute_set_value(self.raw, name.as_ptr(), value.as_ptr())
        };
        if status == K_AX_ERROR_SUCCESS {
            Ok(())
        } else {
            Err(AXError::from_status(
                status,
                name.to_string_lossy().as_ref(),
            ))
        }
    }

    pub fn set_string_attribute(&self, name: &str, value: &str) -> Result<(), AXError> {
        let value = AXValue::from_string(value)?;
        self.set_attribute(name, &value)
    }

    pub fn set_bool_attribute(&self, name: &str, value: bool) -> Result<(), AXError> {
        let value = AXValue::from_bool(value);
        self.set_attribute(name, &value)
    }

    pub fn set_i64_attribute(&self, name: &str, value: i64) -> Result<(), AXError> {
        let value = AXValue::from_i64(value);
        self.set_attribute(name, &value)
    }

    pub fn set_f64_attribute(&self, name: &str, value: f64) -> Result<(), AXError> {
        let value = AXValue::from_f64(value);
        self.set_attribute(name, &value)
    }

    pub fn set_point_attribute(&self, name: &str, value: AXPoint) -> Result<(), AXError> {
        let value = AXValue::from_point(value).ok_or(AXError::Failure)?;
        self.set_attribute(name, &value)
    }

    pub fn set_size_attribute(&self, name: &str, value: AXSize) -> Result<(), AXError> {
        let value = AXValue::from_size(value).ok_or(AXError::Failure)?;
        self.set_attribute(name, &value)
    }

    pub fn set_rect_attribute(&self, name: &str, value: AXRect) -> Result<(), AXError> {
        let value = AXValue::from_rect(value).ok_or(AXError::Failure)?;
        self.set_attribute(name, &value)
    }

    pub fn set_range_attribute(&self, name: &str, value: AXRange) -> Result<(), AXError> {
        let value = AXValue::from_range(value).ok_or(AXError::Failure)?;
        self.set_attribute(name, &value)
    }

    pub fn set_element_attribute(&self, name: &str, value: &Self) -> Result<(), AXError> {
        let value = AXValue::from_element(value).ok_or(AXError::Failure)?;
        self.set_attribute(name, &value)
    }

    pub fn copy_multiple_attribute_values(
        &self,
        names: &[&str],
        options: AXCopyMultipleAttributeOptions,
    ) -> Result<Vec<AXValue>, AXError> {
        let (names_storage, raw_names) = internal::make_cstring_vec(names)?;
        let _storage = names_storage;
        let mut raw = core::ptr::null_mut();
        let status = unsafe {
            bridge::ax_attribute::ax_attribute_copy_multiple_values(
                self.raw,
                raw_names.as_ptr(),
                raw_names.len(),
                options.bits(),
                &mut raw,
            )
        };
        if status == K_AX_ERROR_SUCCESS {
            Ok(raw_to_value_vec(raw))
        } else {
            Err(AXError::from_status(
                status,
                "copy_multiple_attribute_values",
            ))
        }
    }

    pub fn parameterized_attribute_names(&self) -> Result<Vec<String>, AXError> {
        let mut raw = core::ptr::null_mut();
        let status = unsafe {
            bridge::ax_attribute::ax_attribute_copy_parameterized_names(self.raw, &mut raw)
        };
        if status != K_AX_ERROR_SUCCESS {
            return Err(AXError::from_status(
                status,
                "parameterized_attribute_names",
            ));
        }
        Ok(raw_to_string_vec(raw))
    }

    pub fn parameterized_attribute(
        &self,
        name: &str,
        parameter: &AXValue,
    ) -> Result<Option<AXValue>, AXError> {
        let name = internal::make_cstring(name)?;
        let mut raw = core::ptr::null_mut();
        let status = unsafe {
            bridge::ax_attribute::ax_attribute_copy_parameterized_value(
                self.raw,
                name.as_ptr(),
                parameter.as_ptr(),
                &mut raw,
            )
        };
        if status == K_AX_ERROR_SUCCESS {
            Ok((!raw.is_null()).then(|| unsafe { AXValue::from_raw(raw) }))
        } else if status == K_AX_ERROR_NO_VALUE {
            Ok(None)
        } else {
            Err(AXError::from_status(
                status,
                name.to_string_lossy().as_ref(),
            ))
        }
    }

    pub fn action_names(&self) -> Result<Vec<String>, AXError> {
        let mut raw = core::ptr::null_mut();
        let status = unsafe { bridge::ax_action::ax_action_copy_names(self.raw, &mut raw) };
        if status != K_AX_ERROR_SUCCESS {
            return Err(AXError::from_status(status, "action_names"));
        }
        Ok(raw_to_string_vec(raw))
    }

    pub fn action_description(&self, action: &str) -> Result<Option<String>, AXError> {
        let action = internal::make_cstring(action)?;
        let mut raw = core::ptr::null_mut();
        let status = unsafe {
            bridge::ax_action::ax_action_copy_description(self.raw, action.as_ptr(), &mut raw)
        };
        if status == K_AX_ERROR_SUCCESS {
            Ok(unsafe { internal::string_from_handle(raw) })
        } else if status == K_AX_ERROR_NO_VALUE {
            Ok(None)
        } else {
            Err(AXError::from_status(
                status,
                action.to_string_lossy().as_ref(),
            ))
        }
    }

    pub fn perform_action(&self, action: &str) -> Result<(), AXError> {
        let action = internal::make_cstring(action)?;
        let status = unsafe { bridge::ax_action::ax_action_perform(self.raw, action.as_ptr()) };
        if status == K_AX_ERROR_SUCCESS {
            Ok(())
        } else {
            Err(AXError::from_status(
                status,
                action.to_string_lossy().as_ref(),
            ))
        }
    }

    pub fn element_at_position(&self, x: f32, y: f32) -> Result<Option<Self>, AXError> {
        let mut raw = core::ptr::null_mut();
        let status = unsafe {
            bridge::ax_ui_element::ax_ui_element_copy_element_at_position(self.raw, x, y, &mut raw)
        };
        if status == K_AX_ERROR_SUCCESS {
            Ok((!raw.is_null()).then(|| unsafe { Self::from_raw(raw) }))
        } else if status == K_AX_ERROR_NO_VALUE {
            Ok(None)
        } else {
            Err(AXError::from_status(status, "element_at_position"))
        }
    }

    pub fn post_keyboard_event(
        &self,
        key_char: u16,
        virtual_key: u16,
        key_down: bool,
    ) -> Result<(), AXError> {
        let status = unsafe {
            bridge::ax_ui_element::ax_ui_element_post_keyboard_event(
                self.raw,
                key_char,
                virtual_key,
                key_down,
            )
        };
        if status == K_AX_ERROR_SUCCESS {
            Ok(())
        } else {
            Err(AXError::from_status(status, "post_keyboard_event"))
        }
    }

    pub(crate) unsafe fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw
    }
}

fn raw_to_value_vec(raw: *mut c_void) -> Vec<AXValue> {
    if raw.is_null() {
        return Vec::new();
    }
    let value = unsafe { AXValue::from_raw(raw) };
    value.as_array().unwrap_or_default()
}

fn raw_to_string_vec(raw: *mut c_void) -> Vec<String> {
    raw_to_value_vec(raw)
        .into_iter()
        .filter_map(|value| value.as_string())
        .collect()
}
