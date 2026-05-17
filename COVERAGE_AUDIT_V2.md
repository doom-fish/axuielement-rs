# axuielement-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 65
VERIFIED: 62
GAPS: 0
EXEMPT: 3
COVERAGE_PCT: 100.0%

## Methodology

This audit enumerates the public surface of the HIServices AXUIElement framework (via `AXUIElement.h` and `AXValue.h` headers in MacOSX26.2.sdk) and cross-references every symbol against the crate's safe Rust API (`src/**/*.rs`) and raw FFI bindings (`src/ffi/` when `raw-ffi` feature is enabled). All 65 SDK public symbols have been verified: 62 are actively wrapped by the crate's safe interface, and 3 are deprecated APIs excluded from coverage per v2 audit rules (marked with `CF_DEPRECATED_MAC(10_x, 10_9)` in the SDK headers). The crate achieves 100% coverage of non-deprecated macOS public surface.

## 🟢 VERIFIED

| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `AXIsProcessTrustedWithOptions` | function | `AXUIElement.h` | `process_trust::is_process_trusted_with_options()`; `ffi::AXIsProcessTrustedWithOptions` |
| `kAXTrustedCheckOptionPrompt` | CFString constant | `AXUIElement.h` | `ProcessTrustOptions { prompt }`; `ffi::kAXTrustedCheckOptionPrompt` |
| `AXIsProcessTrusted` | function | `AXUIElement.h` | `process_trust::is_process_trusted()`; `ffi::AXIsProcessTrusted` |
| `AXUIElementRef` | opaque type | `AXUIElement.h` | `AXUIElement`; `ffi::AXUIElementRef` |
| `AXCopyMultipleAttributeOptions` | options type | `AXUIElement.h` | `AXCopyMultipleAttributeOptions`; `ffi::AXCopyMultipleAttributeOptions` |
| `kAXCopyMultipleAttributeOptionStopOnError` | option constant | `AXUIElement.h` | `AXCopyMultipleAttributeOptions::STOP_ON_ERROR`; `ffi::kAXCopyMultipleAttributeOptionStopOnError` |
| `AXUIElementGetTypeID` | function | `AXUIElement.h` | `AXUIElement::type_id()`; `ffi::AXUIElementGetTypeID` |
| `AXUIElementCopyAttributeNames` | function | `AXUIElement.h` | `AXUIElement::attribute_names()`; `ffi::AXUIElementCopyAttributeNames` |
| `AXUIElementCopyAttributeValue` | function | `AXUIElement.h` | `AXUIElement::attribute()` and typed accessors; `ffi::AXUIElementCopyAttributeValue` |
| `AXUIElementGetAttributeValueCount` | function | `AXUIElement.h` | `AXUIElement::attribute_value_count()`; `ffi::AXUIElementGetAttributeValueCount` |
| `AXUIElementCopyAttributeValues` | function | `AXUIElement.h` | `AXUIElement::value_array_attribute_range()`; `AXUIElement::element_array_attribute_range()`; `ffi::AXUIElementCopyAttributeValues` |
| `AXUIElementIsAttributeSettable` | function | `AXUIElement.h` | `AXUIElement::is_attribute_settable()`; `ffi::AXUIElementIsAttributeSettable` |
| `AXUIElementSetAttributeValue` | function | `AXUIElement.h` | `AXUIElement::set_attribute()` and typed setters; `ffi::AXUIElementSetAttributeValue` |
| `AXUIElementCopyMultipleAttributeValues` | function | `AXUIElement.h` | `AXUIElement::copy_multiple_attribute_values()`; `ffi::AXUIElementCopyMultipleAttributeValues` |
| `AXUIElementCopyParameterizedAttributeNames` | function | `AXUIElement.h` | `AXUIElement::parameterized_attribute_names()`; `ffi::AXUIElementCopyParameterizedAttributeNames` |
| `AXUIElementCopyParameterizedAttributeValue` | function | `AXUIElement.h` | `AXUIElement::parameterized_attribute()`; `ffi::AXUIElementCopyParameterizedAttributeValue` |
| `AXUIElementCopyActionNames` | function | `AXUIElement.h` | `AXUIElement::action_names()`; `ffi::AXUIElementCopyActionNames` |
| `AXUIElementCopyActionDescription` | function | `AXUIElement.h` | `AXUIElement::action_description()`; `ffi::AXUIElementCopyActionDescription` |
| `AXUIElementPerformAction` | function | `AXUIElement.h` | `AXUIElement::perform_action()`; `ffi::AXUIElementPerformAction` |
| `AXUIElementCopyElementAtPosition` | function | `AXUIElement.h` | `AXUIElement::element_at_position()`; `ffi::AXUIElementCopyElementAtPosition` |
| `AXUIElementCreateApplication` | function | `AXUIElement.h` | `AXUIElement::from_pid()`; `ffi::AXUIElementCreateApplication` |
| `AXUIElementCreateSystemWide` | function | `AXUIElement.h` | `AXUIElement::system_wide()`; `system_wide()`; `ffi::AXUIElementCreateSystemWide` |
| `AXUIElementGetPid` | function | `AXUIElement.h` | `AXUIElement::pid()`; `ffi::AXUIElementGetPid` |
| `AXUIElementSetMessagingTimeout` | function | `AXUIElement.h` | `AXUIElement::set_timeout()`; `ffi::AXUIElementSetMessagingTimeout` |
| `AXTextMarkerRef` | opaque type | `AXUIElement.h (Text Marker API)` | `AXTextMarker`; `ffi::AXTextMarkerRef` |
| `AXTextMarkerGetTypeID` | function | `AXUIElement.h (Text Marker API)` | `AXTextMarker::type_id()`; `ffi::AXTextMarkerGetTypeID` |
| `AXTextMarkerCreate` | function | `AXUIElement.h (Text Marker API)` | `AXTextMarker::from_bytes()`; `ffi::AXTextMarkerCreate` |
| `AXTextMarkerGetLength` | function | `AXUIElement.h (Text Marker API)` | `AXTextMarker::len()`; `ffi::AXTextMarkerGetLength` |
| `AXTextMarkerGetBytePtr` | function | `AXUIElement.h (Text Marker API)` | `AXTextMarker::bytes()` via swift-bridge; `ffi::AXTextMarkerGetBytePtr` |
| `AXTextMarkerRangeRef` | opaque type | `AXUIElement.h (Text Marker API)` | `AXTextMarkerRange`; `ffi::AXTextMarkerRangeRef` |
| `AXTextMarkerRangeGetTypeID` | function | `AXUIElement.h (Text Marker API)` | `AXTextMarkerRange::type_id()`; `ffi::AXTextMarkerRangeGetTypeID` |
| `AXTextMarkerRangeCreate` | function | `AXUIElement.h (Text Marker API)` | `AXTextMarkerRange::new()`; `ffi::AXTextMarkerRangeCreate` |
| `AXTextMarkerRangeCreateWithBytes` | function | `AXUIElement.h (Text Marker API)` | `AXTextMarkerRange::from_bytes()`; `ffi::AXTextMarkerRangeCreateWithBytes` |
| `AXTextMarkerRangeCopyStartMarker` | function | `AXUIElement.h (Text Marker API)` | `AXTextMarkerRange::start_marker()`; `ffi::AXTextMarkerRangeCopyStartMarker` |
| `AXTextMarkerRangeCopyEndMarker` | function | `AXUIElement.h (Text Marker API)` | `AXTextMarkerRange::end_marker()`; `ffi::AXTextMarkerRangeCopyEndMarker` |
| `AXObserverRef` | opaque type | `AXUIElement.h (Notification API)` | `AXObserver`; `ffi::AXObserverRef` |
| `AXObserverCallback` | callback type | `AXUIElement.h (Notification API)` | `ffi::AXObserverCallback`; safe `AXObserver::new()` closure bridge |
| `AXObserverCallbackWithInfo` | callback type | `AXUIElement.h (Notification API)` | `ffi::AXObserverCallbackWithInfo`; safe `AXObserver::new_with_info()` closure bridge |
| `AXObserverGetTypeID` | function | `AXUIElement.h (Notification API)` | `AXObserver::type_id()`; `ffi::AXObserverGetTypeID` |
| `AXObserverCreate` | function | `AXUIElement.h (Notification API)` | `AXObserver::new()`; `ffi::AXObserverCreate` |
| `AXObserverCreateWithInfoCallback` | function | `AXUIElement.h (Notification API)` | `AXObserver::new_with_info()`; `ffi::AXObserverCreateWithInfoCallback` |
| `AXObserverAddNotification` | function | `AXUIElement.h (Notification API)` | `AXObserver::add_notification()`; `ffi::AXObserverAddNotification` |
| `AXObserverRemoveNotification` | function | `AXUIElement.h (Notification API)` | `ffi::AXObserverRemoveNotification`; `AXObserver` drop cleanup |
| `AXObserverGetRunLoopSource` | function | `AXUIElement.h (Notification API)` | `AXObserver::schedule_on_current_run_loop()`; `AXObserver::unschedule_from_run_loop()`; `ffi::AXObserverGetRunLoopSource` |
| `AXValueType` | enum type | `AXValue.h` | `ffi::AXValueType`; `AXValueKind` |
| `kAXValueTypeCGPoint` | enum case | `AXValue.h` | `ffi::kAXValueTypeCGPoint` |
| `kAXValueTypeCGSize` | enum case | `AXValue.h` | `ffi::kAXValueTypeCGSize` |
| `kAXValueTypeCGRect` | enum case | `AXValue.h` | `ffi::kAXValueTypeCGRect` |
| `kAXValueTypeCFRange` | enum case | `AXValue.h` | `ffi::kAXValueTypeCFRange` |
| `kAXValueTypeAXError` | enum case | `AXValue.h` | `ffi::kAXValueTypeAXError` |
| `kAXValueTypeIllegal` | enum case | `AXValue.h` | `ffi::kAXValueTypeIllegal` |
| `kAXValueCGPointType` | legacy constant | `AXValue.h` | `ffi::kAXValueCGPointType` |
| `kAXValueCGSizeType` | legacy constant | `AXValue.h` | `ffi::kAXValueCGSizeType` |
| `kAXValueCGRectType` | legacy constant | `AXValue.h` | `ffi::kAXValueCGRectType` |
| `kAXValueCFRangeType` | legacy constant | `AXValue.h` | `ffi::kAXValueCFRangeType` |
| `kAXValueAXErrorType` | legacy constant | `AXValue.h` | `ffi::kAXValueAXErrorType` |
| `kAXValueIllegalType` | legacy constant | `AXValue.h` | `ffi::kAXValueIllegalType` |
| `AXValueRef` | opaque type | `AXValue.h` | `AXValue`; `ffi::AXValueRef` |
| `AXValueGetTypeID` | function | `AXValue.h` | `AXValue::type_id()`; `ffi::AXValueGetTypeID` |
| `AXValueCreate` | function | `AXValue.h` | `AXValue::from_point()`; `AXValue::from_size()`; `AXValue::from_rect()`; `AXValue::from_range()`; `AXValue::from_error()`; `ffi::AXValueCreate` |
| `AXValueGetType` | function | `AXValue.h` | `AXValue::kind()` via swift-bridge; `ffi::AXValueGetType` |
| `AXValueGetValue` | function | `AXValue.h` | `AXValue::as_point()`; `AXValue::as_size()`; `AXValue::as_rect()`; `AXValue::as_range()`; `AXValue::as_error()`; `ffi::AXValueGetValue` |

## 🔴 GAPS

| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| _None_ | | | All public non-deprecated symbols are covered. |

## ⏭️ EXEMPT

| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `AXAPIEnabled` | function | `AXUIElement.h` | Deprecated API; use `AXIsProcessTrusted` or `AXIsProcessTrustedWithOptions` instead. Crate still exposes via `process_trust::api_enabled()` and `ffi::AXAPIEnabled`. | `CF_DEPRECATED_MAC(10_0, 10_9)` |
| `AXMakeProcessTrusted` | function | `AXUIElement.h` | Deprecated API since OS X 10.9. Crate still wraps via `process_trust::make_process_trusted()` and `ffi::AXMakeProcessTrusted`. | `CF_DEPRECATED_MAC(10_4, 10_9)` |
| `AXUIElementPostKeyboardEvent` | function | `AXUIElement.h` | Deprecated API since OS X 10.9. Crate still wraps via `AXUIElement::post_keyboard_event()` and `ffi::AXUIElementPostKeyboardEvent`. | `CF_DEPRECATED_MAC(10_0, 10_9)` |
