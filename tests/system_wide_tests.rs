use axuielement::{system_wide, SystemWideElement};

#[test]
fn system_wide_element_exists() {
    assert!(system_wide().is_some());
    assert!(SystemWideElement::new().is_some());
}
