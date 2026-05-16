//! Compatibility shim re-exporting the v0.6 element APIs.

pub use crate::ax_ui_element::*;
pub use crate::ax_value::{AXPoint, AXRange, AXRect, AXSize, AXValue, AXValueKind};
pub use crate::process_trust::{
    api_enabled, is_process_trusted, is_process_trusted_with_options,
    is_process_trusted_with_prompt, make_process_trusted, ProcessTrustOptions,
};
pub use crate::system_wide::{system_wide, SystemWideElement};
