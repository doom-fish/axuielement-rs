//! High-level `AXUIElement` wrapper for driving other apps' UIs.

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;

use crate::error::AXError;
use crate::ffi;

/// Owns a retained `AXUIElementRef`. Drops on scope exit.
pub struct AXElement {
    raw: ffi::AXUIElementRef,
}

unsafe impl Send for AXElement {}
unsafe impl Sync for AXElement {}

impl Drop for AXElement {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw.cast_const()) };
            self.raw = ptr::null_mut();
        }
    }
}

impl AXElement {
    #[must_use]
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::AXUIElementGetTypeID() }
    }

    /// Create an element representing the running application with the given PID.
    #[must_use]
    pub fn from_pid(pid: i32) -> Option<Self> {
        let raw = unsafe { ffi::AXUIElementCreateApplication(pid) };
        if raw.is_null() {
            None
        } else {
            Some(Self { raw })
        }
    }

    /// Create the system-wide root element.
    #[must_use]
    pub fn system_wide() -> Option<Self> {
        let raw = unsafe { ffi::AXUIElementCreateSystemWide() };
        if raw.is_null() {
            None
        } else {
            Some(Self { raw })
        }
    }

    /// Set per-element messaging timeout (seconds).
    ///
    /// # Errors
    ///
    /// Returns [`AXError`] propagated from `AXUIElementSetMessagingTimeout`.
    pub fn set_timeout(&self, timeout_seconds: f32) -> Result<(), AXError> {
        let s = unsafe { ffi::AXUIElementSetMessagingTimeout(self.raw, timeout_seconds) };
        if s == ffi::kAXErrorSuccess {
            Ok(())
        } else {
            Err(AXError::from_status(s, "set_timeout"))
        }
    }

    /// Retrieve the PID of the application this element belongs to.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn pid(&self) -> Result<i32, AXError> {
        let mut pid: ffi::pid_t = 0;
        let s = unsafe { ffi::AXUIElementGetPid(self.raw, &mut pid) };
        if s == ffi::kAXErrorSuccess {
            Ok(pid)
        } else {
            Err(AXError::from_status(s, "AXUIElementGetPid"))
        }
    }

    /// Raw `AXUIElementRef` pointer — for FFI interop within this crate.
    #[doc(hidden)]
    #[must_use]
    pub const fn as_ptr(&self) -> ffi::AXUIElementRef {
        self.raw
    }

    /// List every attribute name this element exposes.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn attribute_names(&self) -> Result<Vec<String>, AXError> {
        let mut arr: ffi::CFArrayRef = ptr::null();
        let s = unsafe { ffi::AXUIElementCopyAttributeNames(self.raw, &mut arr) };
        if s != ffi::kAXErrorSuccess {
            return Err(AXError::from_status(s, "attribute_names"));
        }
        let out = cf_array_to_strings(arr);
        if !arr.is_null() {
            unsafe { ffi::CFRelease(arr) };
        }
        Ok(out)
    }

    /// Return whether the attribute is settable.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn is_attribute_settable(&self, name: &str) -> Result<bool, AXError> {
        let cf_name = make_cfstring(name)?;
        let mut settable = false;
        let s = unsafe { ffi::AXUIElementIsAttributeSettable(self.raw, cf_name, &mut settable) };
        unsafe { ffi::CFRelease(cf_name) };
        if s == ffi::kAXErrorSuccess {
            Ok(settable)
        } else if s == ffi::kAXErrorNoValue {
            Ok(false)
        } else {
            Err(AXError::from_status(s, name))
        }
    }

    /// Count the number of values in an array-valued attribute.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn attribute_value_count(&self, name: &str) -> Result<usize, AXError> {
        let cf_name = make_cfstring(name)?;
        let mut count = 0;
        let s = unsafe { ffi::AXUIElementGetAttributeValueCount(self.raw, cf_name, &mut count) };
        unsafe { ffi::CFRelease(cf_name) };
        if s == ffi::kAXErrorSuccess {
            usize::try_from(count)
                .map_err(|_| AXError::IllegalArgument(format!("count overflow for {name}")))
        } else if s == ffi::kAXErrorNoValue {
            Ok(0)
        } else {
            Err(AXError::from_status(s, name))
        }
    }

    /// Read an attribute as a string. Returns `None` if the attribute exists but isn't a string-type.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn string_attribute(&self, name: &str) -> Result<Option<String>, AXError> {
        let cf_name = make_cfstring(name)?;
        let mut value: ffi::CFTypeRef = ptr::null();
        let s = unsafe { ffi::AXUIElementCopyAttributeValue(self.raw, cf_name, &mut value) };
        unsafe { ffi::CFRelease(cf_name) };
        if s != ffi::kAXErrorSuccess {
            if s == ffi::kAXErrorNoValue {
                return Ok(None);
            }
            return Err(AXError::from_status(s, name));
        }
        let out = cf_string_to_string(value);
        unsafe { ffi::CFRelease(value) };
        Ok(out)
    }

    /// Read an attribute as a child element. Returns `None` if the attribute exists but isn't a UI element.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn element_attribute(&self, name: &str) -> Result<Option<Self>, AXError> {
        let cf_name = make_cfstring(name)?;
        let mut value: ffi::CFTypeRef = ptr::null();
        let s = unsafe { ffi::AXUIElementCopyAttributeValue(self.raw, cf_name, &mut value) };
        unsafe { ffi::CFRelease(cf_name) };
        if s != ffi::kAXErrorSuccess {
            if s == ffi::kAXErrorNoValue {
                return Ok(None);
            }
            return Err(AXError::from_status(s, name));
        }
        if value.is_null() {
            return Ok(None);
        }
        if unsafe { ffi::CFGetTypeID(value) } != Self::type_id() {
            unsafe { ffi::CFRelease(value) };
            return Ok(None);
        }
        Ok(Some(Self {
            raw: value.cast_mut(),
        }))
    }

    /// Read a slice of an array-valued attribute and keep only UI elements.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn element_array_attribute_range(
        &self,
        name: &str,
        index: usize,
        max_values: usize,
    ) -> Result<Vec<Self>, AXError> {
        let arr = copy_array_attribute(self.raw, name, index, max_values)?;
        let out = cf_array_to_elements(arr);
        if !arr.is_null() {
            unsafe { ffi::CFRelease(arr) };
        }
        Ok(out)
    }

    /// Read an entire array-valued attribute and keep only UI elements.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn element_array_attribute(&self, name: &str) -> Result<Vec<Self>, AXError> {
        let count = self.attribute_value_count(name)?;
        if count == 0 {
            return Ok(Vec::new());
        }
        self.element_array_attribute_range(name, 0, count)
    }

    /// Convenience helper for `AXChildren`.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn children(&self) -> Result<Vec<Self>, AXError> {
        self.element_array_attribute(ffi::kAXChildrenAttribute)
    }

    /// List every parameterized attribute name this element exposes.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn parameterized_attribute_names(&self) -> Result<Vec<String>, AXError> {
        let mut arr: ffi::CFArrayRef = ptr::null();
        let s = unsafe { ffi::AXUIElementCopyParameterizedAttributeNames(self.raw, &mut arr) };
        if s != ffi::kAXErrorSuccess {
            return Err(AXError::from_status(s, "parameterized_attribute_names"));
        }
        let out = cf_array_to_strings(arr);
        if !arr.is_null() {
            unsafe { ffi::CFRelease(arr) };
        }
        Ok(out)
    }

    /// List every action name this element supports.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn action_names(&self) -> Result<Vec<String>, AXError> {
        let mut arr: ffi::CFArrayRef = ptr::null();
        let s = unsafe { ffi::AXUIElementCopyActionNames(self.raw, &mut arr) };
        if s != ffi::kAXErrorSuccess {
            return Err(AXError::from_status(s, "action_names"));
        }
        let out = cf_array_to_strings(arr);
        if !arr.is_null() {
            unsafe { ffi::CFRelease(arr) };
        }
        Ok(out)
    }

    /// Return Apple's localized description for an action name.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn action_description(&self, action: &str) -> Result<Option<String>, AXError> {
        let cf_action = make_cfstring(action)?;
        let mut description: ffi::CFStringRef = ptr::null();
        let s =
            unsafe { ffi::AXUIElementCopyActionDescription(self.raw, cf_action, &mut description) };
        unsafe { ffi::CFRelease(cf_action) };
        if s != ffi::kAXErrorSuccess {
            return Err(AXError::from_status(s, action));
        }
        let out = cf_string_to_string(description.cast());
        if !description.is_null() {
            unsafe { ffi::CFRelease(description.cast()) };
        }
        Ok(out)
    }

    /// Perform an action by name.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn perform_action(&self, action: &str) -> Result<(), AXError> {
        let cf_action = make_cfstring(action)?;
        let s = unsafe { ffi::AXUIElementPerformAction(self.raw, cf_action) };
        unsafe { ffi::CFRelease(cf_action) };
        if s == ffi::kAXErrorSuccess {
            Ok(())
        } else {
            Err(AXError::from_status(s, action))
        }
    }

    /// Hit-test the element at screen coordinates `(x, y)`.
    ///
    /// # Errors
    ///
    /// Returns [`AXError`] on Apple-side failure. Returns `Ok(None)` if no element is at the requested position.
    pub fn element_at_position(&self, x: f32, y: f32) -> Result<Option<Self>, AXError> {
        let mut out: ffi::AXUIElementRef = core::ptr::null_mut();
        let s = unsafe { ffi::AXUIElementCopyElementAtPosition(self.raw, x, y, &mut out) };
        if s == ffi::kAXErrorSuccess && !out.is_null() {
            Ok(Some(Self { raw: out }))
        } else if s == ffi::kAXErrorNoValue {
            Ok(None)
        } else {
            Err(AXError::from_status(s, "element_at_position"))
        }
    }
}

/// Whether the current process has been granted Accessibility permission.
#[must_use]
pub fn is_process_trusted() -> bool {
    unsafe { ffi::AXIsProcessTrusted() }
}

/// Whether the system-wide Accessibility API is enabled at all.
#[must_use]
pub fn api_enabled() -> bool {
    unsafe { ffi::AXAPIEnabled() }
}

// ---- AXValue typed reads / writes ----

/// A CG point as a typed AX value pair.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AXPoint {
    pub x: f64,
    pub y: f64,
}

/// A CG size as a typed AX value pair.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AXSize {
    pub width: f64,
    pub height: f64,
}

/// A CG rect as a typed AX value pair.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct AXRect {
    pub origin: AXPoint,
    pub size: AXSize,
}

impl AXElement {
    /// Read a `kAXValueTypeCGPoint`-typed attribute.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn point_attribute(&self, name: &str) -> Result<Option<AXPoint>, AXError> {
        let cf_name = make_cfstring(name)?;
        let mut value: ffi::CFTypeRef = ptr::null();
        let s = unsafe { ffi::AXUIElementCopyAttributeValue(self.raw, cf_name, &mut value) };
        unsafe { ffi::CFRelease(cf_name) };
        if s != ffi::kAXErrorSuccess {
            if s == ffi::kAXErrorNoValue {
                return Ok(None);
            }
            return Err(AXError::from_status(s, name));
        }
        if value.is_null() {
            return Ok(None);
        }
        if unsafe { ffi::AXValueGetType(value) } != ffi::kAXValueTypeCGPoint {
            unsafe { ffi::CFRelease(value) };
            return Ok(None);
        }
        let mut point = AXPoint::default();
        let ok = unsafe {
            ffi::AXValueGetValue(
                value,
                ffi::kAXValueTypeCGPoint,
                core::ptr::from_mut(&mut point).cast(),
            )
        };
        unsafe { ffi::CFRelease(value) };
        if ok {
            Ok(Some(point))
        } else {
            Ok(None)
        }
    }

    /// Read a `kAXValueTypeCGSize`-typed attribute.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn size_attribute(&self, name: &str) -> Result<Option<AXSize>, AXError> {
        let cf_name = make_cfstring(name)?;
        let mut value: ffi::CFTypeRef = ptr::null();
        let s = unsafe { ffi::AXUIElementCopyAttributeValue(self.raw, cf_name, &mut value) };
        unsafe { ffi::CFRelease(cf_name) };
        if s != ffi::kAXErrorSuccess {
            if s == ffi::kAXErrorNoValue {
                return Ok(None);
            }
            return Err(AXError::from_status(s, name));
        }
        if value.is_null() {
            return Ok(None);
        }
        if unsafe { ffi::AXValueGetType(value) } != ffi::kAXValueTypeCGSize {
            unsafe { ffi::CFRelease(value) };
            return Ok(None);
        }
        let mut size = AXSize::default();
        let ok = unsafe {
            ffi::AXValueGetValue(
                value,
                ffi::kAXValueTypeCGSize,
                core::ptr::from_mut(&mut size).cast(),
            )
        };
        unsafe { ffi::CFRelease(value) };
        if ok {
            Ok(Some(size))
        } else {
            Ok(None)
        }
    }

    /// Write a `kAXValueTypeCGPoint`-typed attribute.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn set_point_attribute(&self, name: &str, value: AXPoint) -> Result<(), AXError> {
        let cf_name = make_cfstring(name)?;
        let v = unsafe {
            ffi::AXValueCreate(ffi::kAXValueTypeCGPoint, core::ptr::from_ref(&value).cast())
        };
        if v.is_null() {
            unsafe { ffi::CFRelease(cf_name) };
            return Err(AXError::Failure);
        }
        let s = unsafe { ffi::AXUIElementSetAttributeValue(self.raw, cf_name, v) };
        unsafe {
            ffi::CFRelease(v);
            ffi::CFRelease(cf_name);
        }
        if s == ffi::kAXErrorSuccess {
            Ok(())
        } else {
            Err(AXError::from_status(s, name))
        }
    }

    /// Write a `kAXValueTypeCGSize`-typed attribute.
    ///
    /// # Errors
    ///
    /// See [`AXError`].
    pub fn set_size_attribute(&self, name: &str, value: AXSize) -> Result<(), AXError> {
        let cf_name = make_cfstring(name)?;
        let v = unsafe {
            ffi::AXValueCreate(ffi::kAXValueTypeCGSize, core::ptr::from_ref(&value).cast())
        };
        if v.is_null() {
            unsafe { ffi::CFRelease(cf_name) };
            return Err(AXError::Failure);
        }
        let s = unsafe { ffi::AXUIElementSetAttributeValue(self.raw, cf_name, v) };
        unsafe {
            ffi::CFRelease(v);
            ffi::CFRelease(cf_name);
        }
        if s == ffi::kAXErrorSuccess {
            Ok(())
        } else {
            Err(AXError::from_status(s, name))
        }
    }
}

fn copy_array_attribute(
    raw: ffi::AXUIElementRef,
    name: &str,
    index: usize,
    max_values: usize,
) -> Result<ffi::CFArrayRef, AXError> {
    let cf_name = make_cfstring(name)?;
    let index = ffi::CFIndex::try_from(index)
        .map_err(|_| AXError::IllegalArgument(format!("index overflow for {name}")))?;
    let max_values = ffi::CFIndex::try_from(max_values)
        .map_err(|_| AXError::IllegalArgument(format!("max_values overflow for {name}")))?;
    let mut values: ffi::CFArrayRef = ptr::null();
    let s = unsafe {
        ffi::AXUIElementCopyAttributeValues(raw, cf_name, index, max_values, &mut values)
    };
    unsafe { ffi::CFRelease(cf_name) };
    if s == ffi::kAXErrorSuccess {
        Ok(values)
    } else if s == ffi::kAXErrorNoValue {
        Ok(ptr::null())
    } else {
        Err(AXError::from_status(s, name))
    }
}

fn make_cfstring(s: &str) -> Result<ffi::CFStringRef, AXError> {
    let c = CString::new(s).map_err(|e| AXError::IllegalArgument(format!("CString: {e}")))?;
    let cf = unsafe {
        ffi::CFStringCreateWithCString(
            ffi::kCFAllocatorDefault,
            c.as_ptr(),
            ffi::kCFStringEncodingUTF8,
        )
    };
    if cf.is_null() {
        return Err(AXError::IllegalArgument(format!(
            "CFStringCreateWithCString failed for {s:?}"
        )));
    }
    Ok(cf)
}

fn cf_array_to_strings(array: ffi::CFArrayRef) -> Vec<String> {
    if array.is_null() {
        return Vec::new();
    }
    let count = unsafe { ffi::CFArrayGetCount(array) };
    let mut out = Vec::with_capacity(usize::try_from(count).unwrap_or(0));
    for i in 0..count {
        let value = unsafe { ffi::CFArrayGetValueAtIndex(array, i) };
        if let Some(string) = cf_string_to_string(value) {
            out.push(string);
        }
    }
    out
}

fn cf_array_to_elements(array: ffi::CFArrayRef) -> Vec<AXElement> {
    if array.is_null() {
        return Vec::new();
    }
    let count = unsafe { ffi::CFArrayGetCount(array) };
    let mut out = Vec::with_capacity(usize::try_from(count).unwrap_or(0));
    for i in 0..count {
        let value = unsafe { ffi::CFArrayGetValueAtIndex(array, i) };
        if value.is_null() || unsafe { ffi::CFGetTypeID(value) } != AXElement::type_id() {
            continue;
        }
        let retained = unsafe { ffi::CFRetain(value) };
        out.push(AXElement {
            raw: retained.cast_mut(),
        });
    }
    out
}

fn cf_string_to_string(v: *const c_void) -> Option<String> {
    if v.is_null() {
        return None;
    }
    if unsafe { ffi::CFGetTypeID(v) } != unsafe { ffi::CFStringGetTypeID() } {
        return None;
    }
    let s = v as ffi::CFStringRef;
    let len = unsafe { ffi::CFStringGetLength(s) };
    let cap = (len * 4) + 1;
    let mut buf = vec![0u8; usize::try_from(cap).unwrap_or(0)];
    let ok = unsafe {
        ffi::CFStringGetCString(
            s,
            buf.as_mut_ptr().cast::<c_char>(),
            cap,
            ffi::kCFStringEncodingUTF8,
        )
    };
    if !ok {
        return None;
    }
    if let Some(end) = buf.iter().position(|&b| b == 0) {
        buf.truncate(end);
    }
    String::from_utf8(buf).ok()
}
