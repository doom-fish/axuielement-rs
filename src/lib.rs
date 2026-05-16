#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [`AXUIElement`](https://developer.apple.com/documentation/applicationservices/axuielement_h)
//! Accessibility API on macOS — read attributes, list children, perform
//! actions, work with text markers, and subscribe to notifications on other
//! applications' UIs.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::no_effect_underscore_binding,
    clippy::redundant_pub_crate,
    clippy::use_self
)]

mod bridge;
mod internal;

pub mod ax_action;
pub mod ax_attribute;
pub mod ax_error;
pub mod ax_notification;
pub mod ax_observer;
pub mod ax_text_marker;
pub mod ax_ui_element;
pub mod ax_value;
pub mod element;
pub mod error;
#[cfg(feature = "raw-ffi")]
#[cfg_attr(docsrs, doc(cfg(feature = "raw-ffi")))]
pub mod ffi;
pub mod observer;
pub mod process_trust;
pub mod system_wide;

pub use ax_error::AXError;
pub use ax_notification::AXPriority;
pub use ax_observer::{run_current_run_loop, stop_current_run_loop, AXObserver, AXObserverEvent};
pub use ax_text_marker::{AXTextMarker, AXTextMarkerRange};
pub use ax_ui_element::{AXCopyMultipleAttributeOptions, AXElement, AXUIElement};
pub use ax_value::{AXPoint, AXRange, AXRect, AXSize, AXValue, AXValueKind};
pub use process_trust::{
    api_enabled, is_process_trusted, is_process_trusted_with_options,
    is_process_trusted_with_prompt, make_process_trusted, ProcessTrustOptions,
};
pub use system_wide::{system_wide, SystemWideElement};

/// Common imports.
pub mod prelude {
    pub use crate::ax_action;
    pub use crate::ax_attribute;
    pub use crate::ax_notification;
    pub use crate::ax_observer::{
        run_current_run_loop, stop_current_run_loop, AXObserver, AXObserverEvent,
    };
    pub use crate::ax_text_marker::{AXTextMarker, AXTextMarkerRange};
    pub use crate::ax_ui_element::{AXCopyMultipleAttributeOptions, AXElement, AXUIElement};
    pub use crate::ax_value::{AXPoint, AXRange, AXRect, AXSize, AXValue, AXValueKind};
    pub use crate::process_trust::{
        api_enabled, is_process_trusted, is_process_trusted_with_options,
        is_process_trusted_with_prompt, make_process_trusted, ProcessTrustOptions,
    };
    pub use crate::system_wide::{system_wide, SystemWideElement};
    pub use crate::{AXError, AXPriority};
}
