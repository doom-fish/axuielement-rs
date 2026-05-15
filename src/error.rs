//! Errors returned by the `axuielement` crate.

use core::fmt;

use crate::ffi;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AXError {
    Failure,
    IllegalArgument(String),
    InvalidUIElement,
    CannotComplete,
    AttributeUnsupported(String),
    ActionUnsupported(String),
    NotificationUnsupported,
    NotImplemented,
    APIDisabled,
    NoValue,
    /// Raw IOReturn-style error code.
    Other(i32),
}

impl AXError {
    pub(crate) fn from_status(s: i32, ctx: &str) -> Self {
        match s {
            ffi::kAXErrorFailure => Self::Failure,
            ffi::kAXErrorIllegalArgument => Self::IllegalArgument(ctx.to_string()),
            ffi::kAXErrorInvalidUIElement => Self::InvalidUIElement,
            ffi::kAXErrorCannotComplete => Self::CannotComplete,
            ffi::kAXErrorAttributeUnsupported => Self::AttributeUnsupported(ctx.to_string()),
            ffi::kAXErrorActionUnsupported => Self::ActionUnsupported(ctx.to_string()),
            ffi::kAXErrorNotificationUnsupported => Self::NotificationUnsupported,
            ffi::kAXErrorNotImplemented => Self::NotImplemented,
            ffi::kAXErrorAPIDisabled => Self::APIDisabled,
            ffi::kAXErrorNoValue => Self::NoValue,
            other => Self::Other(other),
        }
    }
}

impl fmt::Display for AXError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failure => write!(f, "AX failure"),
            Self::IllegalArgument(c) => write!(f, "illegal argument: {c}"),
            Self::InvalidUIElement => write!(f, "invalid UI element"),
            Self::CannotComplete => write!(f, "cannot complete"),
            Self::AttributeUnsupported(c) => write!(f, "attribute unsupported: {c}"),
            Self::ActionUnsupported(c) => write!(f, "action unsupported: {c}"),
            Self::NotificationUnsupported => write!(f, "notification unsupported"),
            Self::NotImplemented => write!(f, "not implemented"),
            Self::APIDisabled => write!(
                f,
                "Accessibility API disabled — open System Settings → Privacy & Security → Accessibility"
            ),
            Self::NoValue => write!(f, "no value"),
            Self::Other(s) => write!(f, "AXError {s}"),
        }
    }
}

impl std::error::Error for AXError {}
