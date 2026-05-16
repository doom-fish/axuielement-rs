use axuielement::ax_attribute;
use axuielement::AXUIElement;

#[test]
fn application_element_round_trips_pid() {
    let pid = i32::try_from(std::process::id()).expect("current pid fits in i32");
    let app = AXUIElement::from_pid(pid).expect("application element");
    assert_eq!(app.pid().expect("pid"), pid);
}

#[test]
fn system_wide_attribute_names_include_role() {
    let system = AXUIElement::system_wide().expect("system wide element");
    let names = system.attribute_names().expect("attribute names");
    assert!(names
        .iter()
        .any(|name| name == axuielement::ax_attribute::AX_ROLE_ATTRIBUTE));
    assert!(ax_attribute::attributes::ALL_ATTRIBUTES.contains(&ax_attribute::AX_ROLE_ATTRIBUTE));
}
