//! Errors returned by the `axuielement` crate.

use core::fmt;

use crate::{bridge, internal};

pub(crate) const K_AX_ERROR_SUCCESS: i32 = 0;
pub(crate) const K_AX_ERROR_FAILURE: i32 = -25_200;
pub(crate) const K_AX_ERROR_ILLEGAL_ARGUMENT: i32 = -25_201;
pub(crate) const K_AX_ERROR_INVALID_UI_ELEMENT: i32 = -25_202;
pub(crate) const K_AX_ERROR_INVALID_UI_ELEMENT_OBSERVER: i32 = -25_203;
pub(crate) const K_AX_ERROR_CANNOT_COMPLETE: i32 = -25_204;
pub(crate) const K_AX_ERROR_ATTRIBUTE_UNSUPPORTED: i32 = -25_205;
pub(crate) const K_AX_ERROR_ACTION_UNSUPPORTED: i32 = -25_206;
pub(crate) const K_AX_ERROR_NOTIFICATION_UNSUPPORTED: i32 = -25_207;
pub(crate) const K_AX_ERROR_NOT_IMPLEMENTED: i32 = -25_208;
pub(crate) const K_AX_ERROR_NOTIFICATION_ALREADY_REGISTERED: i32 = -25_209;
pub(crate) const K_AX_ERROR_NOTIFICATION_NOT_REGISTERED: i32 = -25_210;
pub(crate) const K_AX_ERROR_API_DISABLED: i32 = -25_211;
pub(crate) const K_AX_ERROR_NO_VALUE: i32 = -25_212;
pub(crate) const K_AX_ERROR_PARAMETERIZED_ATTRIBUTE_UNSUPPORTED: i32 = -25_213;
pub(crate) const K_AX_ERROR_NOT_ENOUGH_PRECISION: i32 = -25_214;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// `ApplicationServices` `AXError` statuses surfaced by the safe wrapper.
pub enum AXError {
    /// Mirrors `kAXErrorFailure`.
    Failure,
    /// Mirrors `kAXErrorIllegalArgument` and carries the failing Rust-side context.
    IllegalArgument(String),
    /// Mirrors `kAXErrorInvalidUIElement`.
    InvalidUIElement,
    /// Mirrors `kAXErrorInvalidUIElementObserver`.
    InvalidUIElementObserver,
    /// Mirrors `kAXErrorCannotComplete`.
    CannotComplete,
    /// Mirrors `kAXErrorAttributeUnsupported` and carries the unsupported attribute name.
    AttributeUnsupported(String),
    /// Mirrors `kAXErrorActionUnsupported` and carries the unsupported action name.
    ActionUnsupported(String),
    /// Mirrors `kAXErrorNotificationUnsupported`.
    NotificationUnsupported,
    /// Mirrors `kAXErrorNotImplemented`.
    NotImplemented,
    /// Mirrors `kAXErrorNotificationAlreadyRegistered`.
    NotificationAlreadyRegistered,
    /// Mirrors `kAXErrorNotificationNotRegistered`.
    NotificationNotRegistered,
    /// Mirrors `kAXErrorAPIDisabled`.
    APIDisabled,
    /// Mirrors `kAXErrorNoValue`.
    NoValue,
    /// Mirrors `kAXErrorParameterizedAttributeUnsupported` and carries the unsupported parameterized attribute name.
    ParameterizedAttributeUnsupported(String),
    /// Mirrors `kAXErrorNotEnoughPrecision`.
    NotEnoughPrecision,
    /// Represents an unrecognized `ApplicationServices` `AXError` status code.
    Other(i32),
}

impl AXError {
    pub(crate) fn from_status(status: i32, context: &str) -> Self {
        match status {
            K_AX_ERROR_FAILURE => Self::Failure,
            K_AX_ERROR_ILLEGAL_ARGUMENT => Self::IllegalArgument(context.to_string()),
            K_AX_ERROR_INVALID_UI_ELEMENT => Self::InvalidUIElement,
            K_AX_ERROR_INVALID_UI_ELEMENT_OBSERVER => Self::InvalidUIElementObserver,
            K_AX_ERROR_CANNOT_COMPLETE => Self::CannotComplete,
            K_AX_ERROR_ATTRIBUTE_UNSUPPORTED => Self::AttributeUnsupported(context.to_string()),
            K_AX_ERROR_ACTION_UNSUPPORTED => Self::ActionUnsupported(context.to_string()),
            K_AX_ERROR_NOTIFICATION_UNSUPPORTED => Self::NotificationUnsupported,
            K_AX_ERROR_NOT_IMPLEMENTED => Self::NotImplemented,
            K_AX_ERROR_NOTIFICATION_ALREADY_REGISTERED => Self::NotificationAlreadyRegistered,
            K_AX_ERROR_NOTIFICATION_NOT_REGISTERED => Self::NotificationNotRegistered,
            K_AX_ERROR_API_DISABLED => Self::APIDisabled,
            K_AX_ERROR_NO_VALUE => Self::NoValue,
            K_AX_ERROR_PARAMETERIZED_ATTRIBUTE_UNSUPPORTED => {
                Self::ParameterizedAttributeUnsupported(context.to_string())
            }
            K_AX_ERROR_NOT_ENOUGH_PRECISION => Self::NotEnoughPrecision,
            other => Self::Other(other),
        }
    }

    #[must_use]
    /// Returns the raw `ApplicationServices` `AXError` code for this value.
    pub const fn raw_code(&self) -> i32 {
        match self {
            Self::Failure => K_AX_ERROR_FAILURE,
            Self::IllegalArgument(_) => K_AX_ERROR_ILLEGAL_ARGUMENT,
            Self::InvalidUIElement => K_AX_ERROR_INVALID_UI_ELEMENT,
            Self::InvalidUIElementObserver => K_AX_ERROR_INVALID_UI_ELEMENT_OBSERVER,
            Self::CannotComplete => K_AX_ERROR_CANNOT_COMPLETE,
            Self::AttributeUnsupported(_) => K_AX_ERROR_ATTRIBUTE_UNSUPPORTED,
            Self::ActionUnsupported(_) => K_AX_ERROR_ACTION_UNSUPPORTED,
            Self::NotificationUnsupported => K_AX_ERROR_NOTIFICATION_UNSUPPORTED,
            Self::NotImplemented => K_AX_ERROR_NOT_IMPLEMENTED,
            Self::NotificationAlreadyRegistered => K_AX_ERROR_NOTIFICATION_ALREADY_REGISTERED,
            Self::NotificationNotRegistered => K_AX_ERROR_NOTIFICATION_NOT_REGISTERED,
            Self::APIDisabled => K_AX_ERROR_API_DISABLED,
            Self::NoValue => K_AX_ERROR_NO_VALUE,
            Self::ParameterizedAttributeUnsupported(_) => {
                K_AX_ERROR_PARAMETERIZED_ATTRIBUTE_UNSUPPORTED
            }
            Self::NotEnoughPrecision => K_AX_ERROR_NOT_ENOUGH_PRECISION,
            Self::Other(code) => *code,
        }
    }

    #[must_use]
    /// Wraps the `ApplicationServices` AX error-description lookup for this status code.
    pub fn localized_description(&self) -> String {
        // SAFETY: FFI call with valid arguments
        let handle = unsafe { bridge::ax_error::ax_error_copy_description(self.raw_code()) };
        // SAFETY: FFI function with valid arguments
        unsafe { internal::string_from_handle(handle) }.unwrap_or_else(|| self.to_string())
    }
}

impl fmt::Display for AXError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failure => write!(f, "AX failure"),
            Self::IllegalArgument(context) => write!(f, "illegal argument: {context}"),
            Self::InvalidUIElement => write!(f, "invalid UI element"),
            Self::InvalidUIElementObserver => write!(f, "invalid UI element observer"),
            Self::CannotComplete => write!(f, "cannot complete"),
            Self::AttributeUnsupported(context) => write!(f, "attribute unsupported: {context}"),
            Self::ActionUnsupported(context) => write!(f, "action unsupported: {context}"),
            Self::NotificationUnsupported => write!(f, "notification unsupported"),
            Self::NotImplemented => write!(f, "not implemented"),
            Self::NotificationAlreadyRegistered => {
                write!(f, "notification already registered")
            }
            Self::NotificationNotRegistered => write!(f, "notification not registered"),
            Self::APIDisabled => write!(
                f,
                "Accessibility API disabled — open System Settings → Privacy & Security → Accessibility",
            ),
            Self::NoValue => write!(f, "no value"),
            Self::ParameterizedAttributeUnsupported(context) => {
                write!(f, "parameterized attribute unsupported: {context}")
            }
            Self::NotEnoughPrecision => write!(f, "not enough precision"),
            Self::Other(status) => write!(f, "AXError {status}"),
        }
    }
}

impl std::error::Error for AXError {}
