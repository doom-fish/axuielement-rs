# axuielement

Safe Rust bindings for Apple's [`AXUIElement`](https://developer.apple.com/documentation/applicationservices/axuielement_h) Accessibility API on macOS.

> **Status:** v0.10 makes `AXObserver` safe to drop from any thread or from inside its own callback, schedules observers in the common run-loop modes, and adds `AXObserver::remove_notification`. The Swift bridge over the C Accessibility APIs covers ten logical areas: `AXUIElement`, `AXObserver`, `AXValue`, `AXTextMarker`, `AXAttribute`, `AXAction`, `AXNotification`, `AXError`, `SystemWide`, and `ProcessTrust`, plus an optional executor-agnostic `async_api` module for observer notifications.

Requires macOS 10.13 or later.

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
- `AXObserver` creation, add/remove notification registration, run-loop scheduling in the common modes, info-dictionary callbacks, and `async_api::AXNotificationStream`.
- `AXValue` round-tripping for strings, booleans, numbers, `CGPoint`, `CGSize`, `CGRect`, `CFRange`, `AXError`, arrays, dictionaries, and binary payloads.
- `AXTextMarker` and `AXTextMarkerRange` creation plus byte round-trips.
- Generated constant modules for attributes, actions, notifications, roles, subroles, menu-item modifiers, and value constants.
- `SystemWideElement` convenience helpers for focused application/window/UI-element lookups.
- `ProcessTrust` helpers for API-enabled checks and trust queries.

See [`COVERAGE.md`](COVERAGE.md) for the audited SDK surface. The web-content constants of `AXWebConstants.h`, including the text-marker attributes, have no named constants yet; pass their names as strings.

## Observers and threads

An `AXObserver` delivers its callback on the thread whose run loop it was scheduled on with `schedule_on_current_run_loop`, in the common modes, so it keeps firing while that thread tracks a menu or a drag. The observer is `Send`. Dropping it, or calling `unschedule_from_run_loop`, removes its run-loop source; when that run loop belongs to another thread and is running, the call waits (up to two seconds) for a callback already in progress to return. The callback's state stays alive until the source is gone, so dropping an observer inside its own callback is safe.

## Raw FFI

The original raw C declarations remain available behind the `raw-ffi` Cargo feature:

```toml
axuielement = { version = "0.9", default-features = false }
# or keep the default `raw-ffi` feature enabled for `axuielement::ffi`
```

The safe API always talks to the Swift bridge. The `raw-ffi` feature exists for direct low-level interop and compatibility.

## Permissions

Most runtime messaging requires **Accessibility permission** (System Settings → Privacy & Security → Accessibility). Without it, many calls return `AXError::APIDisabled` or `AXError::CannotComplete`.

## Async notifications

Enable the `async` Cargo feature to use `axuielement::async_api::AXNotificationStream`, which wraps `AXObserver` notifications in an executor-agnostic bounded async stream.

```toml
axuielement = { version = "0.9", features = ["async"] }
```

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
- `examples/11_async_notification_stream.rs` *(requires `--features async`)*

The test suite includes one smoke file per logical area plus the raw-FFI coverage harness.

## License

Licensed under either [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
