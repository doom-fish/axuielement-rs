# axuielement

Safe Rust bindings for Apple's [`AXUIElement`](https://developer.apple.com/documentation/applicationservices/axuielement_h) Accessibility API on macOS.

> **Status:** v0.6 ships a Wave-C Swift bridge over the C Accessibility APIs and covers the crate's ten logical areas: `AXUIElement`, `AXObserver`, `AXValue`, `AXTextMarker`, `AXAttribute`, `AXAction`, `AXNotification`, `AXError`, `SystemWide`, and `ProcessTrust`.

## Quick start

```rust,no_run
use axuielement::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(system) = system_wide() else {
        eprintln!("no system-wide accessibility object");
        return Ok(());
    };

    println!("api_enabled = {}", api_enabled());
    println!("is_trusted  = {}", is_process_trusted());

    if let Some(app) = system.focused_application()? {
        println!("focused pid   = {}", app.pid()?);
        println!("focused attrs = {:?}", app.attribute_names()?);
    }

    if let Some(focused) = system.focused_ui_element()? {
        println!("role  = {:?}", focused.string_attribute(axuielement::ax_attribute::AX_ROLE_ATTRIBUTE)?);
        println!("title = {:?}", focused.string_attribute(axuielement::ax_attribute::AX_TITLE_ATTRIBUTE)?);
        println!("actions = {:?}", focused.action_names()?);
    }

    Ok(())
}
```

## Covered areas

- `AXUIElement` creation, attribute reads/writes, hit-testing, batch fetches, parameterized attributes, action metadata, and keyboard-event shims.
- `AXObserver` creation, add/remove notification registration, run-loop scheduling, and info-dictionary callbacks.
- `AXValue` round-tripping for strings, booleans, numbers, `CGPoint`, `CGSize`, `CGRect`, `CFRange`, `AXError`, arrays, dictionaries, and binary payloads.
- `AXTextMarker` and `AXTextMarkerRange` creation plus byte round-trips.
- Generated constant modules for attributes, actions, notifications, roles, subroles, menu-item modifiers, and value constants.
- `SystemWideElement` convenience helpers for focused application/window/UI-element lookups.
- `ProcessTrust` helpers for API-enabled checks and trust queries.

See [`COVERAGE.md`](COVERAGE.md) for the audited SDK surface.

## Raw FFI

The original raw C declarations remain available behind the `raw-ffi` Cargo feature:

```toml
axuielement = { version = "0.6", default-features = false }
# or keep the default `raw-ffi` feature enabled for `axuielement::ffi`
```

The safe API always talks to the Swift bridge. The `raw-ffi` feature exists for direct low-level interop and compatibility.

## Permissions

Most runtime messaging requires **Accessibility permission** (System Settings → Privacy & Security → Accessibility). Without it, many calls return `AXError::APIDisabled` or `AXError::CannotComplete`.

## Examples and tests

- `examples/01_axuielement_basic.rs`
- `examples/02_axobserver_basic.rs`
- `examples/03_axvalue_roundtrip.rs`
- `examples/04_axtextmarker_roundtrip.rs`
- `examples/05_axattribute_catalog.rs`
- `examples/06_axaction_catalog.rs`
- `examples/07_axnotification_catalog.rs`
- `examples/08_axerror_catalog.rs`
- `examples/09_system_wide_snapshot.rs`
- `examples/10_process_trust_status.rs`

The test suite includes one smoke file per logical area plus the raw-FFI coverage harness.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
