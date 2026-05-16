use axuielement::ax_action;

#[test]
fn action_catalog_contains_standard_actions() {
    assert!(ax_action::ALL_ACTIONS.contains(&ax_action::AX_PRESS_ACTION));
    assert!(ax_action::ALL_ACTIONS.contains(&ax_action::AX_SHOW_MENU_ACTION));
}
