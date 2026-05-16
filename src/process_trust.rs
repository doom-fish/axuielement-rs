//! Accessibility process-trust helpers.

use crate::ax_error::{AXError, K_AX_ERROR_SUCCESS};
use crate::{bridge, internal};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ProcessTrustOptions {
    pub prompt: bool,
}

impl ProcessTrustOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self { prompt: false }
    }

    #[must_use]
    pub const fn with_prompt(prompt: bool) -> Self {
        Self { prompt }
    }
}

#[must_use]
pub fn api_enabled() -> bool {
    unsafe { bridge::process_trust::ax_process_trust_api_enabled() }
}

#[must_use]
pub fn is_process_trusted() -> bool {
    unsafe { bridge::process_trust::ax_process_trust_is_trusted() }
}

#[must_use]
pub fn is_process_trusted_with_options(options: ProcessTrustOptions) -> bool {
    unsafe { bridge::process_trust::ax_process_trust_is_trusted_with_prompt(options.prompt) }
}

#[must_use]
pub fn is_process_trusted_with_prompt() -> bool {
    is_process_trusted_with_options(ProcessTrustOptions { prompt: true })
}

pub fn make_process_trusted(executable_path: &str) -> Result<(), AXError> {
    let executable_path = internal::make_cstring(executable_path)?;
    let status = unsafe {
        bridge::process_trust::ax_process_trust_make_process_trusted(executable_path.as_ptr())
    };
    if status == K_AX_ERROR_SUCCESS {
        Ok(())
    } else {
        Err(AXError::from_status(
            status,
            executable_path.to_string_lossy().as_ref(),
        ))
    }
}
