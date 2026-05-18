//! System-wide Accessibility helpers.

use core::ops::Deref;

use crate::ax_error::{AXError, K_AX_ERROR_NO_VALUE, K_AX_ERROR_SUCCESS};
use crate::ax_ui_element::AXUIElement;
use crate::bridge;

#[derive(Clone, Debug)]
/// Wrapper around the system-wide `AXUIElementRef` returned by `AXUIElementCreateSystemWide`.
pub struct SystemWideElement {
    inner: AXUIElement,
}

impl Deref for SystemWideElement {
    type Target = AXUIElement;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl SystemWideElement {
    #[must_use]
    /// Wraps `AXUIElementCreateSystemWide`.
    pub fn new() -> Option<Self> {
        // SAFETY: FFI call with valid arguments
        let raw = unsafe { bridge::system_wide::ax_system_wide_create() };
        (!raw.is_null()).then(|| Self {
            // SAFETY: pointer is guaranteed valid from the bridge
            inner: unsafe { AXUIElement::from_raw(raw) },
        })
    }

    /// Convenience wrapper over `AXUIElementCopyAttributeValue` for `kAXFocusedApplicationAttribute`.
    pub fn focused_application(&self) -> Result<Option<AXUIElement>, AXError> {
        copy_system_element(
            self.inner.as_ptr(),
            bridge::system_wide::ax_system_wide_copy_focused_application,
        )
    }

    /// Convenience wrapper over `AXUIElementCopyAttributeValue` for `kAXFocusedUIElementAttribute`.
    pub fn focused_ui_element(&self) -> Result<Option<AXUIElement>, AXError> {
        copy_system_element(
            self.inner.as_ptr(),
            bridge::system_wide::ax_system_wide_copy_focused_ui_element,
        )
    }

    /// Convenience wrapper over `AXUIElementCopyAttributeValue` for `kAXFocusedWindowAttribute`.
    pub fn focused_window(&self) -> Result<Option<AXUIElement>, AXError> {
        copy_system_element(
            self.inner.as_ptr(),
            bridge::system_wide::ax_system_wide_copy_focused_window,
        )
    }

    #[must_use]
    /// Returns the wrapped `AXUIElement` created by `AXUIElementCreateSystemWide`.
    pub fn into_inner(self) -> AXUIElement {
        self.inner
    }
}

#[must_use]
/// Returns the system-wide accessibility element from `AXUIElementCreateSystemWide`.
pub fn system_wide() -> Option<SystemWideElement> {
    SystemWideElement::new()
}

fn copy_system_element(
    handle: *mut core::ffi::c_void,
    callback: unsafe extern "C" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> i32,
) -> Result<Option<AXUIElement>, AXError> {
    let mut raw = core::ptr::null_mut();
    // SAFETY: FFI call with valid arguments
    let status = unsafe { callback(handle, &mut raw) };
    if status == K_AX_ERROR_SUCCESS {
        // SAFETY: pointer is guaranteed valid from the bridge
        Ok((!raw.is_null()).then(|| unsafe { AXUIElement::from_raw(raw) }))
    } else if status == K_AX_ERROR_NO_VALUE {
        Ok(None)
    } else {
        Err(AXError::from_status(status, "system_wide"))
    }
}
