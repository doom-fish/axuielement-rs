//! Print the action constants and, when possible, the focused element's actions.

use axuielement::prelude::*;

fn main() {
    println!("all_actions={:?}", axuielement::ax_action::ALL_ACTIONS);

    if is_process_trusted() {
        if let Some(system) = system_wide() {
            match system.focused_ui_element() {
                Ok(Some(element)) => match element.action_names() {
                    Ok(actions) => println!("focused_actions={actions:?}"),
                    Err(error) => println!("focused_actions=<error: {error}>"),
                },
                Ok(None) => println!("focused_actions=<none>"),
                Err(error) => println!("focused_actions=<error: {error}>"),
            }
        }
    } else {
        println!("focused_actions=<skipped: accessibility not trusted>");
    }
}
