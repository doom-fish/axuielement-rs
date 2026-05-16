//! Snapshot the system-wide focused application/window/UI element when available.

use axuielement::prelude::*;

fn main() {
    let Some(system) = system_wide() else {
        println!("system_wide=<null>");
        return;
    };

    println!("focused_application={:?}", system.focused_application());
    println!("focused_window={:?}", system.focused_window());
    println!("focused_ui_element={:?}", system.focused_ui_element());
}
