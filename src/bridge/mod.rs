#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::c_void;

pub type Handle = *mut c_void;
pub type AXStatus = i32;

pub mod ax_action;
pub mod ax_attribute;
pub mod ax_error;
pub mod ax_notification;
pub mod ax_observer;
pub mod ax_text_marker;
pub mod ax_ui_element;
pub mod ax_value;
pub mod process_trust;
pub mod system_wide;
