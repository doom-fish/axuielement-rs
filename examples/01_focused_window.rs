//! Print the title + role + actions of the currently-focused UI element.
//!
//! Run: `cargo run --example 01_focused_window`
//!
//! REQUIRES Accessibility permission. See README.

use axuielement::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("trust  = {}", is_process_trusted());
    println!("api_on = {}", api_enabled());
    if !is_process_trusted() {
        eprintln!(
            "\nFAILED: not authorised. Add target/debug/examples/01_focused_window\n\
             to System Settings → Privacy & Security → Accessibility, then re-run."
        );
        return Ok(());
    }

    let system = AXElement::system_wide().ok_or("system_wide returned NULL")?;
    let focused = system.element_attribute("AXFocusedUIElement")?
        .ok_or("no focused element")?;

    println!("\n== focused UI element ==");
    println!("  role  : {:?}",  focused.string_attribute("AXRole")?);
    println!("  subrole: {:?}", focused.string_attribute("AXSubrole")?);
    println!("  title : {:?}",  focused.string_attribute("AXTitle")?);
    println!("  desc  : {:?}",  focused.string_attribute("AXDescription")?);
    println!("  attrs : {:?}",  focused.attribute_names()?);
    println!("  actions: {:?}", focused.action_names()?);
    Ok(())
}
