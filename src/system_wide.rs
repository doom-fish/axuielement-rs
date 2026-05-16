//! System-wide Accessibility helpers.

use core::ops::Deref;

use crate::ax_error::{AXError, K_AX_ERROR_NO_VALUE, K_AX_ERROR_SUCCESS};
use crate::ax_ui_element::AXUIElement;
use crate::bridge;

#[derive(Clone, Debug)]
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
    pub fn new() -> Option<Self> {
        let raw = unsafe { bridge::system_wide::ax_system_wide_create() };
        (!raw.is_null()).then(|| Self {
            inner: unsafe { AXUIElement::from_raw(raw) },
        })
    }

    pub fn focused_application(&self) -> Result<Option<AXUIElement>, AXError> {
        copy_system_element(
            self.inner.as_ptr(),
            bridge::system_wide::ax_system_wide_copy_focused_application,
        )
    }

    pub fn focused_ui_element(&self) -> Result<Option<AXUIElement>, AXError> {
        copy_system_element(
            self.inner.as_ptr(),
            bridge::system_wide::ax_system_wide_copy_focused_ui_element,
        )
    }

    pub fn focused_window(&self) -> Result<Option<AXUIElement>, AXError> {
        copy_system_element(
            self.inner.as_ptr(),
            bridge::system_wide::ax_system_wide_copy_focused_window,
        )
    }

    #[must_use]
    pub fn into_inner(self) -> AXUIElement {
        self.inner
    }
}

#[must_use]
pub fn system_wide() -> Option<SystemWideElement> {
    SystemWideElement::new()
}

fn copy_system_element(
    handle: *mut core::ffi::c_void,
    callback: unsafe extern "C" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> i32,
) -> Result<Option<AXUIElement>, AXError> {
    let mut raw = core::ptr::null_mut();
    let status = unsafe { callback(handle, &mut raw) };
    if status == K_AX_ERROR_SUCCESS {
        Ok((!raw.is_null()).then(|| unsafe { AXUIElement::from_raw(raw) }))
    } else if status == K_AX_ERROR_NO_VALUE {
        Ok(None)
    } else {
        Err(AXError::from_status(status, "system_wide"))
    }
}
