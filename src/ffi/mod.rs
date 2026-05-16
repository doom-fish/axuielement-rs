//! Raw FFI declarations for `AXUIElement` (Apple's Accessibility API).
//!
//! Pure C — no Swift bridge.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, missing_docs)]

use core::ffi::{c_char, c_void};

pub type CFTypeRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFArrayRef = *const c_void;
pub type CFIndex = isize;

pub type AXUIElementRef = *mut c_void;
pub type AXError = i32;
pub type pid_t = i32;

pub const kAXErrorSuccess: AXError = 0;
pub const kAXErrorFailure: AXError = -25_200;
pub const kAXErrorIllegalArgument: AXError = -25_201;
pub const kAXErrorInvalidUIElement: AXError = -25_202;
pub const kAXErrorCannotComplete: AXError = -25_204;
pub const kAXErrorAttributeUnsupported: AXError = -25_205;
pub const kAXErrorActionUnsupported: AXError = -25_206;
pub const kAXErrorNotificationUnsupported: AXError = -25_207;
pub const kAXErrorNotImplemented: AXError = -25_208;
pub const kAXErrorNotificationAlreadyRegistered: AXError = -25_209;
pub const kAXErrorAPIDisabled: AXError = -25_211;
pub const kAXErrorNoValue: AXError = -25_212;
pub const kAXErrorParameterizedAttributeUnsupported: AXError = -25_213;

pub const kCFStringEncodingUTF8: u32 = 0x0800_0100;

extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;

    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFGetTypeID(cf: CFTypeRef) -> usize;
    pub fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFStringRef;
    pub fn CFStringGetLength(s: CFStringRef) -> CFIndex;
    pub fn CFStringGetCString(
        s: CFStringRef,
        buffer: *mut c_char,
        buffer_size: CFIndex,
        encoding: u32,
    ) -> bool;
    pub fn CFStringGetTypeID() -> usize;
    pub fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
    pub fn CFArrayGetValueAtIndex(array: CFArrayRef, index: CFIndex) -> *const c_void;

    // AXUIElement
    pub fn AXUIElementCreateApplication(pid: pid_t) -> AXUIElementRef;
    pub fn AXUIElementCreateSystemWide() -> AXUIElementRef;
    pub fn AXUIElementGetPid(element: AXUIElementRef, pid: *mut pid_t) -> AXError;
    pub fn AXUIElementSetMessagingTimeout(element: AXUIElementRef, timeout: f32) -> AXError;

    pub fn AXUIElementCopyAttributeNames(element: AXUIElementRef, names: *mut CFArrayRef) -> AXError;
    pub fn AXUIElementCopyAttributeValue(
        element: AXUIElementRef,
        attribute: CFStringRef,
        value: *mut CFTypeRef,
    ) -> AXError;
    pub fn AXUIElementSetAttributeValue(
        element: AXUIElementRef,
        attribute: CFStringRef,
        value: CFTypeRef,
    ) -> AXError;
    pub fn AXUIElementIsAttributeSettable(
        element: AXUIElementRef,
        attribute: CFStringRef,
        settable: *mut bool,
    ) -> AXError;

    pub fn AXUIElementCopyActionNames(element: AXUIElementRef, names: *mut CFArrayRef) -> AXError;
    pub fn AXUIElementPerformAction(element: AXUIElementRef, action: CFStringRef) -> AXError;
    pub fn AXUIElementCopyElementAtPosition(
        application: AXUIElementRef,
        x: f32,
        y: f32,
        element: *mut AXUIElementRef,
    ) -> AXError;

    // Process trust
    pub fn AXIsProcessTrusted() -> bool;
    pub fn AXAPIEnabled() -> bool;

    // AXValue (typed struct-wrapping CFType)
    pub fn AXValueCreate(value_type: AXValueType, value_ptr: *const c_void) -> CFTypeRef;
    pub fn AXValueGetType(value: CFTypeRef) -> AXValueType;
    pub fn AXValueGetValue(value: CFTypeRef, value_type: AXValueType, value_ptr: *mut c_void) -> bool;

    // AXObserver (notifications)
    pub fn AXObserverCreate(
        application: pid_t,
        callback: AXObserverCallback,
        out_observer: *mut AXObserverRef,
    ) -> AXError;
    pub fn AXObserverAddNotification(
        observer: AXObserverRef,
        element: AXUIElementRef,
        notification: CFStringRef,
        refcon: *mut c_void,
    ) -> AXError;
    pub fn AXObserverRemoveNotification(
        observer: AXObserverRef,
        element: AXUIElementRef,
        notification: CFStringRef,
    ) -> AXError;
    pub fn AXObserverGetRunLoopSource(observer: AXObserverRef) -> *mut c_void;
}

pub type AXValueType = u32;
pub type AXObserverRef = *mut c_void;
pub type AXObserverCallback = unsafe extern "C" fn(
    observer: AXObserverRef,
    element: AXUIElementRef,
    notification: CFStringRef,
    refcon: *mut c_void,
);

pub const kAXValueTypeCGPoint: AXValueType = 1;
pub const kAXValueTypeCGSize: AXValueType = 2;
pub const kAXValueTypeCGRect: AXValueType = 3;
pub const kAXValueTypeCFRange: AXValueType = 4;
pub const kAXValueTypeAXError: AXValueType = 5;
pub const kAXValueTypeIllegal: AXValueType = 0;

extern "C" {
    pub fn CFRunLoopGetCurrent() -> *mut c_void;
    pub fn CFRunLoopAddSource(rl: *mut c_void, source: *mut c_void, mode: CFStringRef);
    pub fn CFRunLoopRemoveSource(rl: *mut c_void, source: *mut c_void, mode: CFStringRef);
    pub fn CFRunLoopRun();
    pub fn CFRunLoopStop(rl: *mut c_void);
    pub static kCFRunLoopCommonModes: CFStringRef;
    pub static kCFRunLoopDefaultMode: CFStringRef;
}

// Common attribute names (defined as CFStrings in AXAttributeConstants.h;
// the underlying string values are stable Apple constants).
pub const kAXRoleAttribute: &str = "AXRole";
pub const kAXSubroleAttribute: &str = "AXSubrole";
pub const kAXTitleAttribute: &str = "AXTitle";
pub const kAXValueAttribute: &str = "AXValue";
pub const kAXDescriptionAttribute: &str = "AXDescription";
pub const kAXChildrenAttribute: &str = "AXChildren";
pub const kAXParentAttribute: &str = "AXParent";
pub const kAXFocusedWindowAttribute: &str = "AXFocusedWindow";
pub const kAXMainWindowAttribute: &str = "AXMainWindow";
pub const kAXFocusedUIElementAttribute: &str = "AXFocusedUIElement";
pub const kAXEnabledAttribute: &str = "AXEnabled";
pub const kAXFocusedAttribute: &str = "AXFocused";
pub const kAXPositionAttribute: &str = "AXPosition";
pub const kAXSizeAttribute: &str = "AXSize";
pub const kAXWindowsAttribute: &str = "AXWindows";

// Common action names.
pub const kAXPressAction: &str = "AXPress";
pub const kAXShowMenuAction: &str = "AXShowMenu";
pub const kAXIncrementAction: &str = "AXIncrement";
pub const kAXDecrementAction: &str = "AXDecrement";
pub const kAXConfirmAction: &str = "AXConfirm";
pub const kAXCancelAction: &str = "AXCancel";
pub const kAXRaiseAction: &str = "AXRaise";
