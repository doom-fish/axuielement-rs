# axuielement

Safe Rust bindings for Apple's [`AXUIElement`](https://developer.apple.com/documentation/applicationservices/axuielement_h) Accessibility API on macOS — read attributes, list children, perform actions on other applications' UIs ("click this button in Photoshop", "what's the title of the focused window?", "list every menu item Safari exposes").

> **Status:** experimental. v0.1 ships construction, attribute / action enumeration + read, perform action, system-wide + per-app entry points. v0.2 adds attribute *write* with typed values, parameterised attributes, observer-based notifications, and child traversal helpers.

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

    // Read the system-wide focused window.
    let system = AXElement::system_wide().ok_or("no system element")?;
    if let Some(window) = system.element_attribute("AXFocusedUIElement")? {
        let title = window.string_attribute("AXTitle")?.unwrap_or_default();
        println!("Focused element title: {title:?}");
        println!("Available attributes: {:?}", window.attribute_names()?);
        println!("Available actions:    {:?}", window.action_names()?);
    }
    Ok(())
}
```

## Permissions

`AXUIElement` requires **Accessibility permission** (System Settings → Privacy & Security → Accessibility). Use [`is_process_trusted`] to check programmatically. Without permission, every API call returns `AXError::APIDisabled` or `AXError::CannotComplete`.

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
- [x] Action enumeration + `perform_action`
- [x] Trust + API-enabled queries
- [x] Per-element messaging timeout
- [ ] Typed attribute writes (string, number, point/size/rect via `AXValue`)
- [ ] Parameterised attribute reads
- [ ] `AXObserver` notifications (run-loop integration)
- [ ] Helpers: walk-children, find-by-role, find-by-title

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
