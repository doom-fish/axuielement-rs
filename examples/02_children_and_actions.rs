//! Print focused-app window metadata using indexed child traversal helpers.
//!
//! Run: `cargo run --example 02_children_and_actions`
//!
//! REQUIRES Accessibility permission. See README.

use axuielement::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("trust  = {}", is_process_trusted());
    println!("api_on = {}", api_enabled());
    if !is_process_trusted() {
        eprintln!(
            "\nFAILED: not authorised. Add target/debug/examples/02_children_and_actions\n\
             to System Settings → Privacy & Security → Accessibility, then re-run."
        );
        return Ok(());
    }

    let Some(system) = AXElement::system_wide() else {
        eprintln!("system_wide returned NULL");
        return Ok(());
    };
    let app = match system.element_attribute("AXFocusedApplication") {
        Ok(Some(app)) => app,
        Ok(None) => {
            eprintln!("no focused application");
            return Ok(());
        }
        Err(err) => {
            eprintln!("focused application lookup failed: {err}");
            return Ok(());
        }
    };

    println!("\n== focused application ==");
    println!("  pid   : {}", app.pid()?);
    match app.attribute_value_count("AXWindows") {
        Ok(count) => println!("  windows: {count}"),
        Err(err) => {
            eprintln!("window count failed: {err}");
            return Ok(());
        }
    }
    println!(
        "  parameterized attrs: {:?}",
        app.parameterized_attribute_names()?
    );

    let windows = match app.element_array_attribute_range("AXWindows", 0, 3) {
        Ok(windows) => windows,
        Err(err) => {
            eprintln!("window lookup failed: {err}");
            return Ok(());
        }
    };
    for (index, window) in windows.into_iter().enumerate() {
        println!("\n== window #{index} ==");
        println!("  role  : {:?}", window.string_attribute("AXRole")?);
        println!("  title : {:?}", window.string_attribute("AXTitle")?);
        println!("  attrs : {:?}", window.attribute_names()?);
        match window.children() {
            Ok(children) => println!("  children: {}", children.len()),
            Err(err) => println!("  children: <error: {err}>"),
        }
        for action in window.action_names()? {
            println!(
                "  action: {action:?} desc={:?}",
                window.action_description(&action)?
            );
        }
    }

    Ok(())
}
