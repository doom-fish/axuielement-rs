//! Safe wrappers for `AXTextMarker` and `AXTextMarkerRange`.

use core::ffi::c_void;
use core::fmt;

use crate::bridge;

#[repr(transparent)]
pub struct AXTextMarker {
    raw: *mut c_void,
}

unsafe impl Send for AXTextMarker {}
unsafe impl Sync for AXTextMarker {}

impl Clone for AXTextMarker {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::ax_text_marker::ax_text_marker_retain(self.raw) };
        unsafe { Self::from_raw(raw) }
    }
}

impl Drop for AXTextMarker {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { bridge::ax_text_marker::ax_text_marker_release(self.raw) };
            self.raw = core::ptr::null_mut();
        }
    }
}

impl fmt::Debug for AXTextMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AXTextMarker")
            .field("len", &self.len())
            .finish()
    }
}

impl AXTextMarker {
    #[must_use]
    pub fn type_id() -> usize {
        unsafe { bridge::ax_text_marker::ax_text_marker_get_type_id() }
    }

    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let raw =
            unsafe { bridge::ax_text_marker::ax_text_marker_create(bytes.as_ptr(), bytes.len()) };
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(raw) })
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        unsafe { bridge::ax_text_marker::ax_text_marker_len(self.raw) }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn bytes(&self) -> Vec<u8> {
        let len = self.len();
        let mut bytes = vec![0_u8; len];
        if len == 0 {
            return bytes;
        }
        let _ = unsafe {
            bridge::ax_text_marker::ax_text_marker_copy_bytes(
                self.raw,
                bytes.as_mut_ptr(),
                bytes.len(),
            )
        };
        bytes
    }

    pub(crate) unsafe fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw
    }
}

#[repr(transparent)]
pub struct AXTextMarkerRange {
    raw: *mut c_void,
}

unsafe impl Send for AXTextMarkerRange {}
unsafe impl Sync for AXTextMarkerRange {}

impl Clone for AXTextMarkerRange {
    fn clone(&self) -> Self {
        let raw = unsafe { bridge::ax_text_marker::ax_text_marker_range_retain(self.raw) };
        unsafe { Self::from_raw(raw) }
    }
}

impl Drop for AXTextMarkerRange {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { bridge::ax_text_marker::ax_text_marker_range_release(self.raw) };
            self.raw = core::ptr::null_mut();
        }
    }
}

impl fmt::Debug for AXTextMarkerRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AXTextMarkerRange").finish()
    }
}

impl AXTextMarkerRange {
    #[must_use]
    pub fn type_id() -> usize {
        unsafe { bridge::ax_text_marker::ax_text_marker_range_get_type_id() }
    }

    #[must_use]
    pub fn new(start: &AXTextMarker, end: &AXTextMarker) -> Option<Self> {
        let raw = unsafe {
            bridge::ax_text_marker::ax_text_marker_range_create(start.as_ptr(), end.as_ptr())
        };
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(raw) })
        }
    }

    #[must_use]
    pub fn from_bytes(start: &[u8], end: &[u8]) -> Option<Self> {
        let raw = unsafe {
            bridge::ax_text_marker::ax_text_marker_range_create_with_bytes(
                start.as_ptr(),
                start.len(),
                end.as_ptr(),
                end.len(),
            )
        };
        if raw.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(raw) })
        }
    }

    #[must_use]
    pub fn start_marker(&self) -> AXTextMarker {
        let raw =
            unsafe { bridge::ax_text_marker::ax_text_marker_range_copy_start_marker(self.raw) };
        unsafe { AXTextMarker::from_raw(raw) }
    }

    #[must_use]
    pub fn end_marker(&self) -> AXTextMarker {
        let raw = unsafe { bridge::ax_text_marker::ax_text_marker_range_copy_end_marker(self.raw) };
        unsafe { AXTextMarker::from_raw(raw) }
    }

    pub(crate) unsafe fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.raw
    }
}
