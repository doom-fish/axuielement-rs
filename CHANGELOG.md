# Changelog

## [0.8.1] - 2026-05-18

### Changed

- Added one-line rustdoc coverage across the safe AXUIElement wrapper, value, observer, notification, text-marker, trust, and constant APIs, bringing rustdoc item coverage to 100%.

## [0.8.0] - 2026-05-18

### Changed

- Breaking: `axuielement::ffi` now re-exports `CFIndex`, `CFTypeID`, `CGCharCode`, and `CGKeyCode` from `apple-cf`'s raw CoreFoundation/CoreGraphics bindings instead of defining local aliases.
- Raised the `apple-cf` dependency requirement to `>=0.9, <0.10` so the crate can share the upstream `CGCharCode` and `CGKeyCode` raw types.

## [0.7.0] - 2026-05-18

### Changed

- Breaking: `axuielement::ffi` now re-exports `CFTypeRef`, `CFStringRef`, `CFAllocatorRef`, `CFArrayRef`, and `CFDictionaryRef` from `apple-cf`'s raw CoreFoundation bindings instead of defining local aliases.
- Added an explicit `apple-cf` dependency with the `cg` feature enabled for shared CoreFoundation/CoreGraphics raw types.

## [0.6.1] - 2025-07-03

### Fixed

- Added SAFETY comments to all 142 unsafe blocks throughout the crate for improved code clarity and auditability.

## [0.6.0] - 2026-05-16

### Added

- Wave-C Swift bridge package (`swift-bridge/`) wrapping the C Accessibility APIs while keeping the raw C surface behind the `raw-ffi` feature.
- Safe modules for all ten logical areas: `AXUIElement`, `AXObserver`, `AXValue`, `AXTextMarker`, `AXAttribute`, `AXAction`, `AXNotification`, `AXError`, `SystemWide`, and `ProcessTrust`.
- Generic `AXValue` conversions for strings, booleans, integers, floating-point values, arrays, dictionaries, binary data, `CGPoint`, `CGSize`, `CGRect`, `CFRange`, `AXError`, `AXUIElement`, `AXTextMarker`, and `AXTextMarkerRange`.
- Generated constant catalogs for attributes, parameterized attributes, roles, subroles, actions, notifications, notification info keys, priorities, menu-item modifiers, and AX value constants.
- Ten numbered examples and one smoke-test file per logical area.
- `COVERAGE.md` documenting the audited ApplicationServices / HIServices SDK surface.

### Changed

- `AXElement` is now a type alias of the Swift-bridge-backed `AXUIElement` wrapper.
- `SystemWideElement` and `ProcessTrustOptions` provide more targeted entry points for focused-element and trust-related workflows.
- README and crate docs now describe the Swift bridge, raw-FFI feature, examples, and audited areas.
- Raw FFI coverage now includes the `AXTextMarker` / `AXTextMarkerRange` APIs and legacy `AXValue` aliases.

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
