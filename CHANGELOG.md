# Changelog

## [0.5.0] - 2026-05-16

### Added

- Full raw FFI coverage for the current `AXUIElement`, `AXValue`, and `AXObserver` C functions, plus trust-related symbols from `AXUIElement.h`.
- `AXElement::{type_id, is_attribute_settable, attribute_value_count, element_array_attribute, element_array_attribute_range, children}`.
- `AXElement::parameterized_attribute_names` and `AXElement::action_description`.
- `AXObserver::type_id`.
- Smoke example `02_children_and_actions` for the new traversal and action-metadata helpers.

### Changed

- `AXPoint`, `AXSize`, and `AXRect` are now `#[repr(C)]` so their `AXValue` interop matches the CoreGraphics layouts they wrap.
- README status/roadmap now reflects the already-shipped point/size writes, hit-testing, and observer support.
- Package contents now include examples and tests.
- Coverage tests now verify `AXUIElement`, `AXValue`, and `AXObserver` header coverage separately.

## [0.1.0] - Initial release

### Added

- `AXElement` opaque wrapper over `AXUIElementRef` (auto-release on Drop).
- Constructors: `from_pid(pid)` + `system_wide()`.
- Attribute enumeration: `attribute_names() -> Vec<String>`.
- Typed reads: `string_attribute(name)`, `element_attribute(name)`.
- Action enumeration + invocation: `action_names()`, `perform_action(name)`.
- Per-element messaging timeout: `set_timeout(secs)`.
- PID lookup: `pid()`.
- Trust queries: `is_process_trusted()`, `api_enabled()`.
- `AXError` enum with the standard `kAXError*` codes.
- 1 example (`01_focused_window`).
- 1 API-coverage test for `AXUIElement` (10 coverable, 100% wrapped; rest documented as v0.2 via the omitted allowlist).

Pure C — zero-Swift bridge.
