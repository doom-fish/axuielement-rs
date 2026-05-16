#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [`AXUIElement`](https://developer.apple.com/documentation/applicationservices/axuielement_h)
//! Accessibility API on macOS — read attributes, list children, perform
//! actions on other applications' UIs.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod element;
pub mod error;
pub mod ffi;

pub use element::{
    api_enabled, is_process_trusted, AXElement, AXPoint, AXRect, AXSize,
};
pub use error::AXError;

/// Common imports.
pub mod prelude {
    pub use crate::element::{
        api_enabled, is_process_trusted, AXElement, AXPoint, AXRect, AXSize,
    };
    pub use crate::error::AXError;
}
