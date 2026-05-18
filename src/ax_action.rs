//! Standard Accessibility action constants.

/// Mirrors the `ApplicationServices` `kAXPressAction` constant.
pub const AX_PRESS_ACTION: &str = "AXPress";
/// Mirrors the `ApplicationServices` `kAXIncrementAction` constant.
pub const AX_INCREMENT_ACTION: &str = "AXIncrement";
/// Mirrors the `ApplicationServices` `kAXDecrementAction` constant.
pub const AX_DECREMENT_ACTION: &str = "AXDecrement";
/// Mirrors the `ApplicationServices` `kAXConfirmAction` constant.
pub const AX_CONFIRM_ACTION: &str = "AXConfirm";
/// Mirrors the `ApplicationServices` `kAXCancelAction` constant.
pub const AX_CANCEL_ACTION: &str = "AXCancel";
/// Mirrors the `ApplicationServices` `kAXShowAlternateUIAction` constant.
pub const AX_SHOW_ALTERNATE_UI_ACTION: &str = "AXShowAlternateUI";
/// Mirrors the `ApplicationServices` `kAXShowDefaultUIAction` constant.
pub const AX_SHOW_DEFAULT_UI_ACTION: &str = "AXShowDefaultUI";
/// Mirrors the `ApplicationServices` `kAXRaiseAction` constant.
pub const AX_RAISE_ACTION: &str = "AXRaise";
/// Mirrors the `ApplicationServices` `kAXShowMenuAction` constant.
pub const AX_SHOW_MENU_ACTION: &str = "AXShowMenu";
/// Mirrors the `ApplicationServices` `kAXPickAction` constant.
pub const AX_PICK_ACTION: &str = "AXPick";

/// Collects the `ApplicationServices` AX action constants exposed by this module.
pub const ALL_ACTIONS: &[&str] = &[
    AX_PRESS_ACTION,
    AX_INCREMENT_ACTION,
    AX_DECREMENT_ACTION,
    AX_CONFIRM_ACTION,
    AX_CANCEL_ACTION,
    AX_SHOW_ALTERNATE_UI_ACTION,
    AX_SHOW_DEFAULT_UI_ACTION,
    AX_RAISE_ACTION,
    AX_SHOW_MENU_ACTION,
    AX_PICK_ACTION,
];
