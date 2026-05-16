//! Print notification constants, info keys, and priority values.

fn main() {
    println!(
        "notifications={}",
        axuielement::ax_notification::ALL_NOTIFICATIONS.len()
    );
    println!("info_keys={:?}", axuielement::ax_notification::INFO_KEYS);
    println!(
        "priorities=({}, {}, {})",
        axuielement::AXPriority::Low.raw(),
        axuielement::AXPriority::Medium.raw(),
        axuielement::AXPriority::High.raw(),
    );
}
