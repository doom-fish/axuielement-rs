//! Generic Accessibility values plus typed geometry helpers.

use core::ffi::c_void;
use core::fmt;

use crate::ax_error::AXError;
use crate::ax_text_marker::{AXTextMarker, AXTextMarkerRange};
use crate::ax_ui_element::AXUIElement;
use crate::{bridge, internal};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AXPoint {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AXSize {
    pub width: f64,
    pub height: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AXRect {
    pub origin: AXPoint,
    pub size: AXSize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AXRange {
    pub location: isize,
    pub length: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AXValueKind {
    Null,
    String,
    Bool,
    Integer,
    Float,
    Element,
    Array,
    Point,
    Size,
    Rect,
    Range,
    Error,
    TextMarker,
    TextMarkerRange,
    Data,
    Dictionary,
    AttributedString,
    Url,
    Unknown,
}

impl AXValueKind {
    fn from_raw(raw: u32) -> Self {
        match raw {
            0 => Self::Null,
            1 => Self::String,
            2 => Self::Bool,
            3 => Self::Integer,
            4 => Self::Float,
            5 => Self::Element,
            6 => Self::Array,
            7 => Self::Point,
            8 => Self::Size,
            9 => Self::Rect,
            10 => Self::Range,
            11 => Self::Error,
            12 => Self::TextMarker,
            13 => Self::TextMarkerRange,
            14 => Self::Data,
            15 => Self::Dictionary,
            16 => Self::AttributedString,
            17 => Self::Url,
            _ => Self::Unknown,
        }
    }
}

#[repr(transparent)]
pub struct AXValue {
    raw: *mut c_void,
}

unsafe impl Send for AXValue {}
unsafe impl Sync for AXValue {}

impl Clone for AXValue {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
        unsafe { Self::from_raw(raw) }
    }
}

impl Drop for AXValue {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { bridge::ax_value::ax_value_release(self.raw) };
            self.raw = core::ptr::null_mut();
        }
    }
}

impl fmt::Debug for AXValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AXValue")
            .field("kind", &self.kind())
            .finish()
    }
}

impl AXValue {
    #[must_use]
    pub fn type_id() -> usize {
        unsafe { bridge::ax_value::ax_value_get_type_id() }
    }

    #[must_use]
    pub fn kind(&self) -> AXValueKind {
        AXValueKind::from_raw(unsafe { bridge::ax_value::ax_value_kind(self.raw) })
    }

    #[must_use]
    pub fn is_null(&self) -> bool {
        self.kind() == AXValueKind::Null
    }

    #[must_use]
    pub fn as_string(&self) -> Option<String> {
        let handle = unsafe { bridge::ax_value::ax_value_copy_string(self.raw) };
        unsafe { internal::string_from_handle(handle) }
    }

    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        let mut value = false;
        unsafe { bridge::ax_value::ax_value_get_bool(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_i64(&self) -> Option<i64> {
        let mut value = 0_i64;
        unsafe { bridge::ax_value::ax_value_get_i64(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        let mut value = 0.0_f64;
        unsafe { bridge::ax_value::ax_value_get_f64(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_point(&self) -> Option<AXPoint> {
        let mut value = AXPoint::default();
        unsafe { bridge::ax_value::ax_value_get_point(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_size(&self) -> Option<AXSize> {
        let mut value = AXSize::default();
        unsafe { bridge::ax_value::ax_value_get_size(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_rect(&self) -> Option<AXRect> {
        let mut value = AXRect::default();
        unsafe { bridge::ax_value::ax_value_get_rect(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_range(&self) -> Option<AXRange> {
        let mut value = AXRange::default();
        unsafe { bridge::ax_value::ax_value_get_range(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_error(&self) -> Option<AXError> {
        let mut status = 0_i32;
        unsafe { bridge::ax_value::ax_value_get_error_code(self.raw, &mut status) }
            .then(|| AXError::from_status(status, "AXValue"))
    }

    #[must_use]
    pub fn as_element(&self) -> Option<AXUIElement> {
        (self.kind() == AXValueKind::Element).then(|| {
            let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
            unsafe { AXUIElement::from_raw(raw) }
        })
    }

    #[must_use]
    pub fn as_text_marker(&self) -> Option<AXTextMarker> {
        (self.kind() == AXValueKind::TextMarker).then(|| {
            let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
            unsafe { AXTextMarker::from_raw(raw) }
        })
    }

    #[must_use]
    pub fn as_text_marker_range(&self) -> Option<AXTextMarkerRange> {
        (self.kind() == AXValueKind::TextMarkerRange).then(|| {
            let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
            unsafe { AXTextMarkerRange::from_raw(raw) }
        })
    }

    #[must_use]
    pub fn as_array(&self) -> Option<Vec<Self>> {
        if self.kind() != AXValueKind::Array {
            return None;
        }
        let len = unsafe { bridge::ax_value::ax_value_array_len(self.raw) };
        let mut values = Vec::with_capacity(len);
        for index in 0..len {
            let raw = unsafe { bridge::ax_value::ax_value_array_get(self.raw, index) };
            if raw.is_null() {
                continue;
            }
            values.push(unsafe { Self::from_raw(raw) });
        }
        Some(values)
    }

    #[must_use]
    pub fn as_dictionary(&self) -> Option<Vec<(String, Self)>> {
        if self.kind() != AXValueKind::Dictionary {
            return None;
        }
        let len = unsafe { bridge::ax_value::ax_value_dictionary_len(self.raw) };
        let mut entries = Vec::with_capacity(len);
        for index in 0..len {
            let key = unsafe {
                internal::string_from_handle(bridge::ax_value::ax_value_dictionary_key_at(
                    self.raw, index,
                ))
            }?;
            let raw = unsafe { bridge::ax_value::ax_value_dictionary_value_at(self.raw, index) };
            if raw.is_null() {
                continue;
            }
            entries.push((key, unsafe { Self::from_raw(raw) }));
        }
        Some(entries)
    }

    #[must_use]
    pub fn as_data(&self) -> Option<Vec<u8>> {
        if self.kind() != AXValueKind::Data {
            return None;
        }
        let len = unsafe { bridge::ax_value::ax_value_data_len(self.raw) };
        let mut bytes = vec![0_u8; len];
        if len == 0 {
            return Some(bytes);
        }
        let ok = unsafe {
            bridge::ax_value::ax_value_copy_data_bytes(self.raw, bytes.as_mut_ptr(), bytes.len())
        };
        ok.then_some(bytes)
    }

    #[must_use]
    pub fn null() -> Self {
        let raw = unsafe { bridge::ax_value::ax_value_create_null() };
        unsafe { Self::from_raw(raw) }
    }

    pub fn from_string(value: &str) -> Result<Self, AXError> {
        let value = internal::make_cstring(value)?;
        let raw = unsafe { bridge::ax_value::ax_value_create_string(value.as_ptr()) };
        if raw.is_null() {
            return Err(AXError::Failure);
        }
        Ok(unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_bool(value: bool) -> Self {
        let raw = unsafe { bridge::ax_value::ax_value_create_bool(value) };
        unsafe { Self::from_raw(raw) }
    }

    #[must_use]
    pub fn from_i64(value: i64) -> Self {
        let raw = unsafe { bridge::ax_value::ax_value_create_i64(value) };
        unsafe { Self::from_raw(raw) }
    }

    #[must_use]
    pub fn from_f64(value: f64) -> Self {
        let raw = unsafe { bridge::ax_value::ax_value_create_f64(value) };
        unsafe { Self::from_raw(raw) }
    }

    #[must_use]
    pub fn from_point(value: AXPoint) -> Option<Self> {
        let raw = unsafe { bridge::ax_value::ax_value_create_point(value) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_size(value: AXSize) -> Option<Self> {
        let raw = unsafe { bridge::ax_value::ax_value_create_size(value) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_rect(value: AXRect) -> Option<Self> {
        let raw = unsafe { bridge::ax_value::ax_value_create_rect(value) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_range(value: AXRange) -> Option<Self> {
        let raw = unsafe { bridge::ax_value::ax_value_create_range(value) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_error(error: AXError) -> Option<Self> {
        let raw = unsafe { bridge::ax_value::ax_value_create_error_code(error.raw_code()) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_element(value: &AXUIElement) -> Option<Self> {
        let raw = unsafe { bridge::ax_value::ax_value_create_element(value.as_ptr()) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_text_marker(value: &AXTextMarker) -> Option<Self> {
        let raw = unsafe { bridge::ax_value::ax_value_create_text_marker(value.as_ptr()) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_text_marker_range(value: &AXTextMarkerRange) -> Option<Self> {
        let raw = unsafe { bridge::ax_value::ax_value_create_text_marker_range(value.as_ptr()) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_data(bytes: &[u8]) -> Option<Self> {
        let raw = unsafe { bridge::ax_value::ax_value_create_data(bytes.as_ptr(), bytes.len()) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    pub fn from_array(values: &[&AXValue]) -> Option<Self> {
        let handles = values
            .iter()
            .map(|value| value.as_ptr())
            .collect::<Vec<_>>();
        let raw =
            unsafe { bridge::ax_value::ax_value_create_array(handles.as_ptr(), handles.len()) };
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    pub fn from_dictionary(entries: &[(&str, &AXValue)]) -> Result<Option<Self>, AXError> {
        let (keys, raw_keys) =
            internal::make_cstring_vec(&entries.iter().map(|(key, _)| *key).collect::<Vec<_>>())?;
        let _keys = keys;
        let values = entries
            .iter()
            .map(|(_, value)| value.as_ptr())
            .collect::<Vec<_>>();
        let raw = unsafe {
            bridge::ax_value::ax_value_create_dictionary(
                raw_keys.as_ptr(),
                values.as_ptr(),
                values.len(),
            )
        };
        Ok((!raw.is_null()).then(|| unsafe { Self::from_raw(raw) }))
    }

    pub(crate) unsafe fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw
    }
}
