# Changelog

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
- 1 API-coverage test for `AXUIElement` (10 coverable, 100% wrapped;
  rest documented as v0.2 via the omitted allowlist).

Pure C — zero-Swift bridge.
