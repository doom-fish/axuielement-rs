//! Create a couple of `AXUIElement` handles and print basic metadata.

use axuielement::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pid = i32::try_from(std::process::id()).expect("current pid fits in i32");
    let current = AXUIElement::from_pid(pid).ok_or("no current AXUIElement")?;
    println!("type_id={}", AXUIElement::type_id());
    println!("current_pid={}", current.pid()?);

    if let Some(system) = AXUIElement::system_wide() {
        println!("system_attributes={:?}", system.attribute_names()?);
    }

    Ok(())
}
