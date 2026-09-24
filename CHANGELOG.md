# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.0] - Unreleased

### Security

- `AXObserver` no longer frees its callback state under a running callback. The callback's context pointer borrowed the observer's state, and `Drop` freed it without synchronising with the run loop delivering callbacks, so dropping an observer on another thread or inside its own callback was a use-after-free. The state is now a `doom_fish_utils::callback_context::CallbackContext` owned by the Swift observer until its run-loop source is gone; a cross-thread drop removes the source and waits for that run loop to finish a callback in progress.

### Fixed

- Dropping an `AXNotificationStream` right after subscribing could hang forever in `join()`: the stop reached the stream's run loop before `CFRunLoopRun` started, and Core Foundation discards such a stop. Stopping now queues a block that stops the loop from inside, so an early or repeated stop is never lost.
- `SystemWideElement::focused_window` reads `AXFocusedWindow` from the focused application; the system-wide element has no such attribute, so it used to fail.
- The observer's run-loop source is scheduled in the common modes instead of only the default mode, so notifications keep arriving while the host tracks a menu or a drag.
- `AXAPIEnabled`, `AXMakeProcessTrusted` and `AXUIElementPostKeyboardEvent` are called through C declarations instead of `@_silgen_name` shims that used the Swift calling convention.
- The build script no longer adds the toolchain's Swift 5.5 back-deployment directory (`usr/lib/swift-5.5/macosx`) to the link search path and rpath. Its old `libswift_Concurrency.dylib` shadowed the SDK's, so a binary that also linked Swift code using newer concurrency APIs failed to link, and the rpath pointed into Xcode, which user machines don't have. The bridge uses no Swift concurrency, so the macOS 10.13 minimum is unchanged.
- README and `COVERAGE*.md`: `AXObserverRemoveNotification` really has a safe wrapper now, the audited SDK is named, `AXWebConstants.h` and its text-marker attributes are listed as not covered, and raw-only `VERIFIED` rows are identified.

### Changed

- **BREAKING:** requires `apple-cf` 0.11 (`>=0.11, <0.12`); the `ffi` module's Core Foundation and Core Graphics type aliases come from it. `doom-fish-utils` (`>=0.4.1, <0.5`) is a regular dependency now, and the `async` feature enables its `futures-stream` feature.
- **BREAKING:** `rust-version` is 1.82.
- **BREAKING:** the `raw-ffi` feature is off by default; enable it to keep using the raw declarations in `axuielement::ffi`. It stayed a default feature only for compatibility, and the safe API never needed it.
- Dropping an `AXObserver`, or calling `unschedule_from_run_loop`, from a thread other than the one whose run loop the observer is scheduled on waits up to two seconds for a callback in progress on that loop.
- Observer callbacks run without an internal lock, so a callback that re-enters its own observer no longer deadlocks; panics are contained by `CallbackContext::with`.

### Added

- `AXObserver::remove_notification`.

## [0.9.1] - 2026-06-06

### Fixed

- Balanced the per-callback observer retain, contained panics in `AXObserver` callbacks, and pinned the `AXValue` geometry ABI with layout tests.

## [0.9.0] - 2026-05-20

### Added

- `async_api` module behind the `async` feature, providing executor-agnostic async wrappers for `AXObserver` notifications via `AXNotificationStream`. Uses `doom-fish-utils::stream`.

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
