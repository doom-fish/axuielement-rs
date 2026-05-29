//! ABI layout assertions for the geometry structs shared with the Swift bridge.
//!
//! `AXPoint`, `AXSize`, `AXRect` and `AXRange` are `#[repr(C)]` mirrors of
//! `CGPoint`, `CGSize`, `CGRect` and `CFRange`. They are passed by value (or via
//! out-pointers) across the Rust <-> Swift `@_cdecl` FFI boundary. If their size
//! or alignment ever drifts from what the Swift side expects, the marshalled
//! geometry silently corrupts. These tests pin the layout so accidental field
//! reordering / type changes are caught at `cargo test` time rather than as
//! runtime garbage.

use std::mem::{align_of, size_of};

use axuielement::{AXPoint, AXRange, AXRect, AXSize};

#[test]
fn ax_point_layout() {
    // 2 x f64
    assert_eq!(size_of::<AXPoint>(), 16, "AXPoint size drifted");
    assert_eq!(align_of::<AXPoint>(), 8, "AXPoint alignment drifted");
}

#[test]
fn ax_size_layout() {
    // 2 x f64
    assert_eq!(size_of::<AXSize>(), 16, "AXSize size drifted");
    assert_eq!(align_of::<AXSize>(), 8, "AXSize alignment drifted");
}

#[test]
fn ax_rect_layout() {
    // AXPoint (16) + AXSize (16)
    assert_eq!(size_of::<AXRect>(), 32, "AXRect size drifted");
    assert_eq!(align_of::<AXRect>(), 8, "AXRect alignment drifted");
}

#[test]
fn ax_range_layout() {
    // 2 x isize (CFIndex)
    assert_eq!(size_of::<AXRange>(), 16, "AXRange size drifted");
    assert_eq!(align_of::<AXRange>(), 8, "AXRange alignment drifted");
}

/// Cross-language ABI check: asks the Swift bridge to verify that *its*
/// `MemoryLayout` (size/stride/alignment) for `CGPoint`, `CGSize`, `CGRect` and
/// `CFRange` matches the values pinned on the Rust side. A `false` return means
/// the Rust and Swift layouts genuinely disagree, which is a real ABI bug.
#[test]
fn ax_value_layout_matches_swift() {
    assert!(
        axuielement::ax_value::verify_layout(),
        "Swift geometry layout disagrees with Rust layout (ABI mismatch)"
    );
}
