//! Create text markers and marker ranges entirely in-process.

use axuielement::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = AXTextMarker::from_bytes(&[1, 2, 3]).ok_or("start marker")?;
    let end = AXTextMarker::from_bytes(&[4, 5, 6]).ok_or("end marker")?;
    let range = AXTextMarkerRange::new(&start, &end).ok_or("range")?;

    println!("start={:?}", start.bytes());
    println!("end={:?}", end.bytes());
    println!("range_start={:?}", range.start_marker().bytes());
    println!("range_end={:?}", range.end_marker().bytes());
    Ok(())
}
