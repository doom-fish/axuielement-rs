//! Generic Accessibility values plus typed geometry helpers.

use core::ffi::c_void;
use core::fmt;

use crate::ax_error::AXError;
use crate::ax_text_marker::{AXTextMarker, AXTextMarkerRange};
use crate::ax_ui_element::AXUIElement;
use crate::{bridge, internal};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
/// Rust representation of the `CGPoint` payload used with `AXValueCreate` and `AXValueGetValue`.
pub struct AXPoint {
    /// Mirrors the `CGPoint.x` coordinate carried by `kAXValueTypeCGPoint`.
    pub x: f64,
    /// Mirrors the `CGPoint.y` coordinate carried by `kAXValueTypeCGPoint`.
    pub y: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
/// Rust representation of the `CGSize` payload used with `AXValueCreate` and `AXValueGetValue`.
pub struct AXSize {
    /// Mirrors the `CGSize.width` field carried by `kAXValueTypeCGSize`.
    pub width: f64,
    /// Mirrors the `CGSize.height` field carried by `kAXValueTypeCGSize`.
    pub height: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
/// Rust representation of the `CGRect` payload used with `AXValueCreate` and `AXValueGetValue`.
pub struct AXRect {
    /// Mirrors the `CGRect.origin` field carried by `kAXValueTypeCGRect`.
    pub origin: AXPoint,
    /// Mirrors the `CGRect.size` field carried by `kAXValueTypeCGRect`.
    pub size: AXSize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
/// Rust representation of the `CFRange` payload used with `AXValueCreate` and `AXValueGetValue`.
pub struct AXRange {
    /// Mirrors the `CFRange.location` field carried by `kAXValueTypeCFRange`.
    pub location: isize,
    /// Mirrors the `CFRange.length` field carried by `kAXValueTypeCFRange`.
    pub length: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
/// Payload kinds carried by `AXValueGetType` and the crate bridge for Accessibility values.
pub enum AXValueKind {
    /// Represents a null placeholder in the bridge value transport.
    Null,
    /// Represents a string payload copied from an Accessibility attribute value.
    String,
    /// Represents a boolean payload copied from an Accessibility attribute value.
    Bool,
    /// Represents an integer payload copied from an Accessibility attribute value.
    Integer,
    /// Represents a floating-point payload copied from an Accessibility attribute value.
    Float,
    /// Represents an `AXUIElementRef` payload copied from an Accessibility attribute value.
    Element,
    /// Represents an array payload copied from an Accessibility attribute value.
    Array,
    /// Represents the `kAXValueTypeCGPoint` payload kind.
    Point,
    /// Represents the `kAXValueTypeCGSize` payload kind.
    Size,
    /// Represents the `kAXValueTypeCGRect` payload kind.
    Rect,
    /// Represents the `kAXValueTypeCFRange` payload kind.
    Range,
    /// Represents the `kAXValueTypeAXError` payload kind.
    Error,
    /// Represents an `AXTextMarkerRef` payload copied from an Accessibility attribute value.
    TextMarker,
    /// Represents an `AXTextMarkerRangeRef` payload copied from an Accessibility attribute value.
    TextMarkerRange,
    /// Represents opaque data copied from an Accessibility attribute value.
    Data,
    /// Represents a dictionary payload copied from an Accessibility attribute value.
    Dictionary,
    /// Represents an attributed-string payload copied from an Accessibility attribute value.
    AttributedString,
    /// Represents a URL payload copied from an Accessibility attribute value.
    Url,
    /// Represents a payload kind the bridge could not map to a known `ApplicationServices` tag.
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
/// Owned wrapper around an `ApplicationServices` `AXValueRef` or bridged Accessibility payload.
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
    /// Wraps `AXValueGetTypeID`.
    pub fn type_id() -> usize {
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_type_id() }
    }

    #[must_use]
    /// Returns the payload kind reported by `AXValueGetType` or the bridge equivalent for non-AXValue payloads.
    pub fn kind(&self) -> AXValueKind {
        // SAFETY: pointer is guaranteed valid from the bridge
        AXValueKind::from_raw(unsafe { bridge::ax_value::ax_value_kind(self.raw) })
    }

    #[must_use]
    /// Returns whether this value is the bridge null placeholder.
    pub fn is_null(&self) -> bool {
        self.kind() == AXValueKind::Null
    }

    #[must_use]
    /// Returns a string payload copied from an Accessibility value.
    pub fn as_string(&self) -> Option<String> {
        // SAFETY: FFI call with valid arguments
        let handle = unsafe { bridge::ax_value::ax_value_copy_string(self.raw) };
        // SAFETY: FFI function with valid arguments
        unsafe { internal::string_from_handle(handle) }
    }

    #[must_use]
    /// Returns a boolean payload copied from an Accessibility value.
    pub fn as_bool(&self) -> Option<bool> {
        let mut value = false;
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_bool(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    /// Returns an integer payload copied from an Accessibility value.
    pub fn as_i64(&self) -> Option<i64> {
        let mut value = 0_i64;
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_i64(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    /// Returns a floating-point payload copied from an Accessibility value.
    pub fn as_f64(&self) -> Option<f64> {
        let mut value = 0.0_f64;
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_f64(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    /// Wraps `AXValueGetValue` for `kAXValueTypeCGPoint`.
    pub fn as_point(&self) -> Option<AXPoint> {
        let mut value = AXPoint::default();
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_point(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    /// Wraps `AXValueGetValue` for `kAXValueTypeCGSize`.
    pub fn as_size(&self) -> Option<AXSize> {
        let mut value = AXSize::default();
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_size(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    /// Wraps `AXValueGetValue` for `kAXValueTypeCGRect`.
    pub fn as_rect(&self) -> Option<AXRect> {
        let mut value = AXRect::default();
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_rect(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    /// Wraps `AXValueGetValue` for `kAXValueTypeCFRange`.
    pub fn as_range(&self) -> Option<AXRange> {
        let mut value = AXRange::default();
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_range(self.raw, &mut value) }.then_some(value)
    }

    #[must_use]
    /// Wraps `AXValueGetValue` for `kAXValueTypeAXError`.
    pub fn as_error(&self) -> Option<AXError> {
        let mut status = 0_i32;
        // SAFETY: FFI boundary with properly validated inputs
        unsafe { bridge::ax_value::ax_value_get_error_code(self.raw, &mut status) }
            .then(|| AXError::from_status(status, "AXValue"))
    }

    #[must_use]
    /// Returns an `AXUIElementRef` payload copied from an Accessibility value.
    pub fn as_element(&self) -> Option<AXUIElement> {
        (self.kind() == AXValueKind::Element).then(|| {
            // SAFETY: FFI call with valid arguments
            let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
            // SAFETY: pointer is guaranteed valid from the bridge
            unsafe { AXUIElement::from_raw(raw) }
        })
    }

    #[must_use]
    /// Returns an `AXTextMarkerRef` payload copied from an Accessibility value.
    pub fn as_text_marker(&self) -> Option<AXTextMarker> {
        (self.kind() == AXValueKind::TextMarker).then(|| {
            // SAFETY: FFI call with valid arguments
            let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
            // SAFETY: pointer is guaranteed valid from the bridge
            unsafe { AXTextMarker::from_raw(raw) }
        })
    }

    #[must_use]
    /// Returns an `AXTextMarkerRangeRef` payload copied from an Accessibility value.
    pub fn as_text_marker_range(&self) -> Option<AXTextMarkerRange> {
        (self.kind() == AXValueKind::TextMarkerRange).then(|| {
            // SAFETY: FFI call with valid arguments
            let raw = unsafe { bridge::ax_value::ax_value_retain(self.raw) };
            // SAFETY: pointer is guaranteed valid from the bridge
            unsafe { AXTextMarkerRange::from_raw(raw) }
        })
    }

    #[must_use]
    /// Returns an array payload copied from an Accessibility value.
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
    /// Returns a dictionary payload copied from an Accessibility value.
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
    /// Returns opaque bytes copied from an Accessibility value.
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
    /// Creates the bridge null placeholder for an empty Accessibility payload.
    pub fn null() -> Self {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_null() };
        // SAFETY: pointer is guaranteed valid from the bridge
        unsafe { Self::from_raw(raw) }
    }

    /// Creates a string payload that can be passed through Accessibility value APIs.
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
    /// Creates a boolean payload that can be passed through Accessibility value APIs.
    pub fn from_bool(value: bool) -> Self {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_bool(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        unsafe { Self::from_raw(raw) }
    }

    #[must_use]
    /// Creates an integer payload that can be passed through Accessibility value APIs.
    pub fn from_i64(value: i64) -> Self {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_i64(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        unsafe { Self::from_raw(raw) }
    }

    #[must_use]
    /// Creates a floating-point payload that can be passed through Accessibility value APIs.
    pub fn from_f64(value: f64) -> Self {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_f64(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        unsafe { Self::from_raw(raw) }
    }

    #[must_use]
    /// Wraps `AXValueCreate` for `kAXValueTypeCGPoint`.
    pub fn from_point(value: AXPoint) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_point(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    /// Wraps `AXValueCreate` for `kAXValueTypeCGSize`.
    pub fn from_size(value: AXSize) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_size(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    /// Wraps `AXValueCreate` for `kAXValueTypeCGRect`.
    pub fn from_rect(value: AXRect) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_rect(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    /// Wraps `AXValueCreate` for `kAXValueTypeCFRange`.
    pub fn from_range(value: AXRange) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_range(value) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    /// Wraps `AXValueCreate` for `kAXValueTypeAXError`.
    pub fn from_error(error: AXError) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_error_code(error.raw_code()) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    /// Boxes an `AXUIElementRef` payload for `AXUIElementSetAttributeValue` and related APIs.
    pub fn from_element(value: &AXUIElement) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_element(value.as_ptr()) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    /// Boxes an `AXTextMarkerRef` payload for Accessibility value APIs.
    pub fn from_text_marker(value: &AXTextMarker) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_text_marker(value.as_ptr()) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    /// Boxes an `AXTextMarkerRangeRef` payload for Accessibility value APIs.
    pub fn from_text_marker_range(value: &AXTextMarkerRange) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_text_marker_range(value.as_ptr()) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    #[must_use]
    /// Boxes opaque bytes for Accessibility value APIs.
    pub fn from_data(bytes: &[u8]) -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::ax_value::ax_value_create_data(bytes.as_ptr(), bytes.len()) };
        // SAFETY: pointer is guaranteed valid from the bridge
        (!raw.is_null()).then(|| unsafe { Self::from_raw(raw) })
    }

    /// Boxes an array payload for Accessibility value APIs.
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

    /// Boxes a dictionary payload for Accessibility value APIs.
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
