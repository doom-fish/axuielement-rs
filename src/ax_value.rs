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
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
        // SAFETY: pointer is guaranteed valid from the bridge
        unsafe { Self::from_raw(raw) }
    }
}

impl Drop for AXValue {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: FFI boundary with properly validated inputs
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
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_type_id() }
    }

    #[must_use]
    pub fn kind(&self) -> AXValueKind {
        // SAFETY: pointer is guaranteed valid from the bridge
        AXValueKind::from_raw(unsafe { bridge::ax_value::ax_value_kind(self.raw) })
    }

    #[must_use]
    pub fn is_null(&self) -> bool {
        self.kind() == AXValueKind::Null
    }

    #[must_use]
    pub fn as_string(&self) -> Option<String> {
        // SAFETY: FFI call with valid arguments
        let handle = unsafe { bridge::ax_value::ax_value_copy_string(self.raw) };
        // SAFETY: FFI function with valid arguments
        unsafe { internal::string_from_handle(handle) }
    }

    #[must_use]
    pub fn as_bool(&self) -> Option<bool> {
        let mut value = false;
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_bool(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_i64(&self) -> Option<i64> {
        let mut value = 0_i64;
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_i64(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_f64(&self) -> Option<f64> {
        let mut value = 0.0_f64;
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_f64(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_point(&self) -> Option<AXPoint> {
        let mut value = AXPoint::default();
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_point(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_size(&self) -> Option<AXSize> {
        let mut value = AXSize::default();
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_size(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_rect(&self) -> Option<AXRect> {
        let mut value = AXRect::default();
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_rect(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_range(&self) -> Option<AXRange> {
        let mut value = AXRange::default();
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_range(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    pub fn as_error(&self) -> Option<AXError> {
        let mut status = 0_i32;
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_error_code(self.raw, &mut status) }
            .then(|| AXError::from_status(status, "AXValue"))
    }

    #[must_use]
    pub fn as_element(&self) -> Option<AXUIElement> {
        (self.kind() == AXValueKind::Element).then(|| {
            // SAFETY: FFI call with valid arguments
            let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
            // SAFETY: pointer is guaranteed valid from the bridge
            unsafe { AXUIElement::from_raw(raw) }
        })
    }

    #[must_use]
    pub fn as_text_marker(&self) -> Option<AXTextMarker> {
        (self.kind() == AXValueKind::TextMarker).then(|| {
            // SAFETY: FFI call with valid arguments
            let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
            // SAFETY: pointer is guaranteed valid from the bridge
            unsafe { AXTextMarker::from_raw(raw) }
        })
    }

    #[must_use]
    pub fn as_text_marker_range(&self) -> Option<AXTextMarkerRange> {
        (self.kind() == AXValueKind::TextMarkerRange).then(|| {
            // SAFETY: FFI call with valid arguments
            let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
            // SAFETY: pointer is guaranteed valid from the bridge
            unsafe { AXTextMarkerRange::from_raw(raw) }
        })
    }

    #[must_use]
    pub fn as_array(&self) -> Option<Vec<Self>> {
        if self.kind() != AXValueKind::Array {
            return None;
        }
        // SAFETY: FFI call with valid arguments
        let len = unsafe { bridge::ax_value::ax_value_array_len(self.raw) };
        let mut values = Vec::with_capacity(len);
        for index in 0..len {
            // SAFETY: FFI call with valid arguments
            let raw = unsafe { bridge::ax_value::ax_value_array_get(self.raw, index) };
            if raw.is_null() {
                continue;
            }
            // SAFETY: pointer is guaranteed valid from the bridge
            values.push(unsafe { Self::from_raw(raw) });
        }
        Some(values)
    }

    #[must_use]
    pub fn as_dictionary(&self) -> Option<Vec<(String, Self)>> {
        if self.kind() != AXValueKind::Dictionary {
            return None;
        }
        // SAFETY: FFI call with valid arguments
        let len = unsafe { bridge::ax_value::ax_value_dictionary_len(self.raw) };
        let mut entries = Vec::with_capacity(len);
        for index in 0..len {
            // SAFETY: FFI call with valid arguments
            let key = unsafe {
                internal::string_from_handle(bridge::ax_value::ax_value_dictionary_key_at(
                    self.raw, index,
                ))
            }?;
            // SAFETY: FFI call with valid arguments
            let raw = unsafe { bridge::ax_value::ax_value_dictionary_value_at(self.raw, index) };
            if raw.is_null() {
                continue;
            }
            // SAFETY: pointer is guaranteed valid from the bridge
            entries.push((key, unsafe { Self::from_raw(raw) }));
        }
        Some(entries)
    }

    #[must_use]
    pub fn as_data(&self) -> Option<Vec<u8>> {
        if self.kind() != AXValueKind::Data {
            return None;
        }
        // SAFETY: FFI call with valid arguments
        let len = unsafe { bridge::ax_value::ax_value_data_len(self.raw) };
        let mut bytes = vec![0_u8; len];
        if len == 0 {
            return Some(bytes);
        }
        // SAFETY: FFI call with valid arguments
        let ok = unsafe {
            bridge::ax_value::ax_value_copy_data_bytes(self.raw, bytes.as_mut_ptr(), bytes.len())
        };
        ok.then_some(bytes)
    }

    #[must_use]
    pub fn null() -> Self {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_null() };
        // SAFETY: pointer is guaranteed valid from the bridge
        unsafe { Self::from_raw(raw) }
    }

    pub fn from_string(value: &str) -> Result<Self, AXError> {
        let value = internal::make_cstring(value)?;
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_string(value.as_ptr()) };
        if raw.is_null() {
            return Err(AXError::Failure);
        }
        // SAFETY: pointer is guaranteed valid from the bridge
        Ok(unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_bool(value: bool) -> Self {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_bool(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        unsafe { Self::from_raw(raw) }
    }

    #[must_use]
    pub fn from_i64(value: i64) -> Self {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_i64(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        unsafe { Self::from_raw(raw) }
    }

    #[must_use]
    pub fn from_f64(value: f64) -> Self {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_f64(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        unsafe { Self::from_raw(raw) }
    }

    #[must_use]
    pub fn from_point(value: AXPoint) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_point(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_size(value: AXSize) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_size(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_rect(value: AXRect) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_rect(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_range(value: AXRange) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_range(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_error(error: AXError) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_error_code(error.raw_code()) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_element(value: &AXUIElement) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_element(value.as_ptr()) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_text_marker(value: &AXTextMarker) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_text_marker(value.as_ptr()) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_text_marker_range(value: &AXTextMarkerRange) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_text_marker_range(value.as_ptr()) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    pub fn from_data(bytes: &[u8]) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_data(bytes.as_ptr(), bytes.len()) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    pub fn from_array(values: &[&AXValue]) -> Option<Self> {
        let handles = values
            .iter()
            .map(|value| value.as_ptr())
            .collect::<Vec<_>>();
        let raw =
            // SAFETY: FFI boundary with properly validated inputs
            unsafe { bridge::ax_value::ax_value_create_array(handles.as_ptr(), handles.len()) };
        // SAFETY: pointer is guaranteed valid from the bridge
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
        // SAFETY: FFI call with valid arguments
        let raw = unsafe {
            bridge::ax_value::ax_value_create_dictionary(
                raw_keys.as_ptr(),
                values.as_ptr(),
                values.len(),
            )
        };
        // SAFETY: pointer is guaranteed valid from the bridge
        Ok((!raw.is_null()).then(|| unsafe { Self::from_raw(raw) }))
    }

    pub(crate) unsafe fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw
    }
}
