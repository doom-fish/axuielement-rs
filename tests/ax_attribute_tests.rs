use axuielement::ax_attribute;

#[test]
fn attribute_catalogs_are_populated() {
    assert!(ax_attribute::attributes::ALL_ATTRIBUTES.contains(&ax_attribute::AX_ROLE_ATTRIBUTE));
    assert!(ax_attribute::parameterized::ALL_PARAMETERIZED_ATTRIBUTES
        .contains(&ax_attribute::AX_LINE_FOR_INDEX_PARAMETERIZED_ATTRIBUTE));
    assert!(ax_attribute::roles::ALL_ROLES.contains(&ax_attribute::AX_APPLICATION_ROLE));
    assert!(
        ax_attribute::subroles::ALL_SUBROLES.contains(&ax_attribute::AX_STANDARD_WINDOW_SUBROLE)
    );
    assert!(ax_attribute::values::ALL_VALUE_CONSTANTS
        .contains(&ax_attribute::AX_HORIZONTAL_ORIENTATION_VALUE));
}

#[test]
fn menu_item_modifier_constants_match_sdk() {
    assert_eq!(ax_attribute::menu_item_modifiers::NONE, 0);
    assert_eq!(ax_attribute::menu_item_modifiers::SHIFT, 1);
    assert_eq!(ax_attribute::menu_item_modifiers::OPTION, 2);
    assert_eq!(ax_attribute::menu_item_modifiers::CONTROL, 4);
    assert_eq!(ax_attribute::menu_item_modifiers::NO_COMMAND, 8);
}
