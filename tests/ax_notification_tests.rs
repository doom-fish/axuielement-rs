use axuielement::ax_notification;
use axuielement::AXPriority;

#[test]
fn notification_catalog_contains_expected_values() {
    assert!(ax_notification::ALL_NOTIFICATIONS.contains(&ax_notification::AX_CREATED_NOTIFICATION));
    assert!(ax_notification::INFO_KEYS.contains(&ax_notification::AX_PRIORITY_KEY));
}

#[test]
fn priority_values_match_sdk() {
    assert_eq!(AXPriority::Low.raw(), 10);
    assert_eq!(AXPriority::Medium.raw(), 50);
    assert_eq!(AXPriority::High.raw(), 90);
}
