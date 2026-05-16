# axuielement

Safe Rust bindings for Apple's [`AXUIElement`](https://developer.apple.com/documentation/applicationservices/axuielement_h) Accessibility API on macOS — read attributes, list children, perform actions, and subscribe to notifications on other applications' UIs.

> **Status:** actively developed. v0.5 ships construction, attribute/action enumeration, string/element/point/size reads, point/size writes, indexed child traversal, action descriptions, parameterized-attribute name enumeration, hit-testing, and `AXObserver` notifications.

Pure C — **zero Swift bridge**.

## Quick start

```rust,no_run
use axuielement::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !is_process_trusted() {
        eprintln!("Open System Settings → Privacy & Security → Accessibility");
        eprintln!("and grant your binary, then re-run.");
        return Ok(());
    }

    let system = AXElement::system_wide().ok_or("no system element")?;
    let app = system
        .element_attribute("AXFocusedApplication")?
        .ok_or("no focused app")?;

    println!("AXWindows count: {}", app.attribute_value_count("AXWindows")?);
    for window in app.element_array_attribute_range("AXWindows", 0, 3)? {
        println!("title={:?}", window.string_attribute("AXTitle")?);
        println!("actions={:?}", window.action_names()?);
        println!("action desc={:?}", window.action_description("AXRaise")?);
    }
    Ok(())
}
```

## Permissions

`AXUIElement` requires **Accessibility permission** (System Settings → Privacy & Security → Accessibility). Use [`is_process_trusted`] to check programmatically. Without permission, most messaging APIs return `AXError::APIDisabled` or `AXError::CannotComplete`.

## Pipeline composition

```text
axuielement (drive UI) ──► click "Save", read window title, walk menu tree
                              │
                              ├─► cgevents (post synthetic input as fallback)
                              ├─► screencapturekit (record what happens)
                              └─► foundation-models ("summarise this dialog")
```

## Roadmap

- [x] `AXElement::{from_pid, system_wide}`
- [x] Attribute enumeration (`attribute_names`)
- [x] Read string + element attributes
- [x] Typed point/size reads + writes via `AXValue`
- [x] Indexed array reads + child traversal helpers (`attribute_value_count`, `element_array_attribute_range`, `children`)
- [x] Action enumeration + descriptions + `perform_action`
- [x] Hit-testing (`element_at_position`)
- [x] Trust + API-enabled queries
- [x] Per-element messaging timeout
- [x] `AXObserver` notifications (run-loop integration)
- [x] Parameterized attribute-name enumeration
- [ ] Generic parameterized-attribute value helpers
- [ ] `AXUIElementCopyMultipleAttributeValues` high-level wrapper
- [ ] `AXObserverCreateWithInfoCallback` high-level wrapper
- [ ] Additional typed attribute writes (`CGRect`, strings, numbers)

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
