//! Round-trip a few values through the generic `AXValue` wrapper.

use axuielement::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let point = AXValue::from_point(AXPoint { x: 10.0, y: 20.0 }).ok_or("point")?;
    let text = AXValue::from_string("hello accessibility")?;
    let number = AXValue::from_i64(42);
    let data = AXValue::from_data(&[1, 2, 3, 4]).ok_or("data")?;
    let array = AXValue::from_array(&[&text, &number, &point]).ok_or("array")?;
    let dictionary = AXValue::from_dictionary(&[("answer", &number), ("payload", &data)])?
        .ok_or("dictionary")?;

    println!("point={:?}", point.as_point());
    println!("array={:?}", array.as_array());
    println!("dictionary={:?}", dictionary.as_dictionary());
    Ok(())
}
