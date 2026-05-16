# COVERAGE

Status for the v0.6.0 Wave-C Swift-bridge implementation. Every public Accessibility SDK symbol in the audited headers is listed below.

## AXUIElement.h

| API | Status | Notes |
| --- | --- | --- |
| `AXObserverRef` | ✅ implemented | Swift bridge handle or callback type. |
| `AXTextMarkerRangeRef` | ✅ implemented | Swift bridge handle or callback type. |
| `AXTextMarkerRef` | ✅ implemented | Swift bridge handle or callback type. |
| `AXUIElementRef` | ✅ implemented | Swift bridge handle or callback type. |
| `AXValueRef` | ✅ implemented | Safe and raw value wrappers expose this type. |
| `AXValueType` | ✅ implemented | Safe and raw value wrappers expose this type. |
| `AXUIElementCopyActionDescription` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementCopyActionNames` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementCopyAttributeNames` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementCopyAttributeValue` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementCopyAttributeValues` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementCopyElementAtPosition` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementCopyMultipleAttributeValues` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementCopyParameterizedAttributeNames` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementCopyParameterizedAttributeValue` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementCreateApplication` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementCreateSystemWide` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementGetAttributeValueCount` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementGetPid` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementGetTypeID` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementIsAttributeSettable` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementPerformAction` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementPostKeyboardEvent` | ✅ implemented | Swift bridge shim wraps the deprecated C symbol. |
| `AXUIElementSetAttributeValue` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXUIElementSetMessagingTimeout` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXTextMarkerCreate` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXTextMarkerGetBytePtr` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXTextMarkerGetLength` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXTextMarkerGetTypeID` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXTextMarkerRangeCopyEndMarker` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXTextMarkerRangeCopyStartMarker` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXTextMarkerRangeCreate` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXTextMarkerRangeCreateWithBytes` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXTextMarkerRangeGetTypeID` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXObserverAddNotification` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXObserverCreate` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXObserverCreateWithInfoCallback` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXObserverGetRunLoopSource` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXObserverGetTypeID` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXObserverRemoveNotification` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXAPIEnabled` | ✅ implemented | Swift bridge shim wraps the deprecated C symbol. |
| `AXIsProcessTrusted` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXIsProcessTrustedWithOptions` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXMakeProcessTrusted` | ✅ implemented | Swift bridge shim wraps the deprecated C symbol. |
| `kAXCopyMultipleAttributeOptionStopOnError` | ✅ implemented | Safe and raw option constants exposed. |
| `kAXTrustedCheckOptionPrompt` | ✅ implemented | ProcessTrust bridge uses the SDK constant. |

## AXError.h

| API | Status | Notes |
| --- | --- | --- |
| `kAXErrorSuccess` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorFailure` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorIllegalArgument` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorInvalidUIElement` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorInvalidUIElementObserver` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorCannotComplete` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorAttributeUnsupported` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorActionUnsupported` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorNotificationUnsupported` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorNotImplemented` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorNotificationAlreadyRegistered` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorNotificationNotRegistered` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorAPIDisabled` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorNoValue` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorParameterizedAttributeUnsupported` | ✅ implemented | Mapped to `AXError` and raw constants. |
| `kAXErrorNotEnoughPrecision` | ✅ implemented | Mapped to `AXError` and raw constants. |

## AXValue.h

| API | Status | Notes |
| --- | --- | --- |
| `AXValueCreate` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXValueGetType` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXValueGetTypeID` | ✅ implemented | Swift bridge + safe wrapper. |
| `AXValueGetValue` | ✅ implemented | Swift bridge + safe wrapper. |
| `kAXValueCGPointType` | ✅ implemented | Legacy value-type alias preserved in the raw FFI. |
| `kAXValueCGSizeType` | ✅ implemented | Legacy value-type alias preserved in the raw FFI. |
| `kAXValueCGRectType` | ✅ implemented | Legacy value-type alias preserved in the raw FFI. |
| `kAXValueCFRangeType` | ✅ implemented | Legacy value-type alias preserved in the raw FFI. |
| `kAXValueAXErrorType` | ✅ implemented | Legacy value-type alias preserved in the raw FFI. |
| `kAXValueIllegalType` | ✅ implemented | Legacy value-type alias preserved in the raw FFI. |
| `kAXValueTypeCGPoint` | ✅ implemented | Safe `AXValueKind` / raw constant coverage. |
| `kAXValueTypeCGSize` | ✅ implemented | Safe `AXValueKind` / raw constant coverage. |
| `kAXValueTypeCGRect` | ✅ implemented | Safe `AXValueKind` / raw constant coverage. |
| `kAXValueTypeCFRange` | ✅ implemented | Safe `AXValueKind` / raw constant coverage. |
| `kAXValueTypeAXError` | ✅ implemented | Safe `AXValueKind` / raw constant coverage. |
| `kAXValueTypeIllegal` | ✅ implemented | Safe `AXValueKind` / raw constant coverage. |

## AXAttributeConstants.h

| API | Status | Notes |
| --- | --- | --- |
| `kAXRoleAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSubroleAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRoleDescriptionAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHelpAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTitleAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXValueAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXValueDescriptionAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMinValueAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMaxValueAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXValueIncrementAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXAllowedValuesAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXPlaceholderValueAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXEnabledAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXElementBusyAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFocusedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXParentAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXChildrenAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedChildrenAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXVisibleChildrenAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXWindowAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTopLevelUIElementAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXPositionAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSizeAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXOrientationAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDescriptionAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDescription` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedTextAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedTextRangeAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedTextRangesAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXVisibleCharacterRangeAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXNumberOfCharactersAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSharedTextUIElementsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSharedCharacterRangeAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSharedFocusElementsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXInsertionPointLineNumberAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMainAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMinimizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXCloseButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXZoomButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMinimizeButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXToolbarButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFullScreenButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXProxyAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXGrowAreaAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXModalAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDefaultButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXCancelButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuItemCmdCharAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuItemCmdVirtualKeyAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuItemCmdGlyphAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuItemCmdModifiersAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuItemMarkCharAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuItemPrimaryUIElementAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuBarAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXWindowsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFrontmostAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHiddenAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMainWindowAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFocusedWindowAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFocusedUIElementAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXExtrasMenuBarAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHeaderAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXEditedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXValueWrapsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTabsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTitleUIElementAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHorizontalScrollBarAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXVerticalScrollBarAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXOverflowButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFilenameAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXExpandedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSplittersAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXNextContentsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDocumentAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDecrementButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXIncrementButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXPreviousContentsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXContentsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXIncrementorAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHourFieldAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMinuteFieldAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSecondFieldAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXAMPMFieldAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDayFieldAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMonthFieldAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXYearFieldAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXColumnTitleAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXURLAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXLabelUIElementsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXLabelValueAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXShownMenuUIElementAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXServesAsTitleForUIElementsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXLinkedUIElementsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRowsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXVisibleRowsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedRowsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXColumnsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXVisibleColumnsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedColumnsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSortDirectionAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXIndexAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDisclosingAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDisclosedRowsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDisclosedByRowAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDisclosureLevelAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMatteHoleAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMatteContentUIElementAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMarkerUIElementsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXUnitsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXUnitDescriptionAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMarkerTypeAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMarkerTypeDescriptionAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXIsApplicationRunningAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSearchButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXClearButtonAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFocusedApplicationAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRowCountAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXColumnCountAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXOrderedByRowAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXWarningValueAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXCriticalValueAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedCellsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXVisibleCellsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRowHeaderUIElementsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXColumnHeaderUIElementsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRowIndexRangeAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXColumnIndexRangeAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHorizontalUnitsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXVerticalUnitsAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHorizontalUnitDescriptionAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXVerticalUnitDescriptionAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHandlesAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTextAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXVisibleTextAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXIsEditableAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXColumnTitlesAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXIdentifierAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXAlternateUIVisibleAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXLineForIndexParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRangeForLineParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXStringForRangeParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRangeForPositionParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRangeForIndexParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXBoundsForRangeParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRTFForRangeParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXAttributedStringForRangeParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXStyleRangeForIndexParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXCellForColumnAndRowParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXLayoutPointForScreenPointParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXLayoutSizeForScreenSizeParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXScreenPointForLayoutPointParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |
| `kAXScreenSizeForLayoutSizeParameterizedAttribute` | ✅ implemented | Exported as a Rust constant module. |

## AXActionConstants.h

| API | Status | Notes |
| --- | --- | --- |
| `kAXPressAction` | ✅ implemented | Exported as a Rust constant module. |
| `kAXIncrementAction` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDecrementAction` | ✅ implemented | Exported as a Rust constant module. |
| `kAXConfirmAction` | ✅ implemented | Exported as a Rust constant module. |
| `kAXCancelAction` | ✅ implemented | Exported as a Rust constant module. |
| `kAXShowAlternateUIAction` | ✅ implemented | Exported as a Rust constant module. |
| `kAXShowDefaultUIAction` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRaiseAction` | ✅ implemented | Exported as a Rust constant module. |
| `kAXShowMenuAction` | ✅ implemented | Exported as a Rust constant module. |
| `kAXPickAction` | ✅ implemented | Exported as a Rust constant module. |

## AXNotificationConstants.h

| API | Status | Notes |
| --- | --- | --- |
| `kAXMainWindowChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFocusedWindowChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFocusedUIElementChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXApplicationActivatedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXApplicationDeactivatedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXApplicationHiddenNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXApplicationShownNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXWindowCreatedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXWindowMovedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXWindowResizedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXWindowMiniaturizedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXWindowDeminiaturizedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDrawerCreatedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSheetCreatedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHelpTagCreatedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXValueChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXUIElementDestroyedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXElementBusyChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuOpenedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuClosedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuItemSelectedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRowCountChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRowExpandedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRowCollapsedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedCellsChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXUnitsChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedChildrenMovedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedChildrenChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXResizedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMovedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXCreatedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedRowsChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedColumnsChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSelectedTextChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTitleChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXLayoutChangedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXAnnouncementRequestedNotification` | ✅ implemented | Exported as a Rust constant module. |
| `kAXUIElementsKey` | ✅ implemented | Exported as a Rust constant module. |
| `kAXPriorityKey` | ✅ implemented | Exported as a Rust constant module. |
| `kAXAnnouncementKey` | ✅ implemented | Exported as a Rust constant module. |
| `kAXUIElementTitleKey` | ✅ implemented | Exported as a Rust constant module. |
| `kAXPriorityLow` | ✅ implemented | Exported as `AXPriority`. |
| `kAXPriorityMedium` | ✅ implemented | Exported as `AXPriority`. |
| `kAXPriorityHigh` | ✅ implemented | Exported as `AXPriority`. |

## AXRoleConstants.h

| API | Status | Notes |
| --- | --- | --- |
| `kAXApplicationRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSystemWideRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXWindowRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSheetRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDrawerRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXGrowAreaRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXImageRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXUnknownRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXButtonRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRadioButtonRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXCheckBoxRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXPopUpButtonRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuButtonRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTabGroupRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTableRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXColumnRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRowRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXOutlineRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXBrowserRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXScrollAreaRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXScrollBarRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRadioGroupRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXListRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXGroupRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXValueIndicatorRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXComboBoxRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSliderRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXIncrementorRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXBusyIndicatorRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXProgressIndicatorRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRelevanceIndicatorRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXToolbarRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDisclosureTriangleRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTextFieldRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTextAreaRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXStaticTextRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHeadingRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuBarRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuBarItemRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMenuItemRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSplitGroupRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSplitterRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXColorWellRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTimeFieldRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDateFieldRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHelpTagRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMatteRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDockItemRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRulerRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRulerMarkerRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXGridRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXLevelIndicatorRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXCellRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXLayoutAreaRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXLayoutItemRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXHandleRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXPopoverRole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXCloseButtonSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMinimizeButtonSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXZoomButtonSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXToolbarButtonSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFullScreenButtonSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSecureTextFieldSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTableRowSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXOutlineRowSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXUnknownSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXStandardWindowSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDialogSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSystemDialogSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFloatingWindowSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSystemFloatingWindowSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDecorativeSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXIncrementArrowSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDecrementArrowSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXIncrementPageSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDecrementPageSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSortButtonSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSearchFieldSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTimelineSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXRatingIndicatorSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXContentListSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDefinitionListSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDescriptionListSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXToggleSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSwitchSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXApplicationDockItemSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDocumentDockItemSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXFolderDockItemSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXMinimizedWindowDockItemSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXURLDockItemSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDockExtraDockItemSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXTrashDockItemSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXSeparatorDockItemSubrole` | ✅ implemented | Exported as a Rust constant module. |
| `kAXProcessSwitcherListSubrole` | ✅ implemented | Exported as a Rust constant module. |

## AXValueConstants.h

| API | Status | Notes |
| --- | --- | --- |
| `kAXHorizontalOrientationValue` | ✅ implemented | Exported as a Rust constant module. |
| `kAXVerticalOrientationValue` | ✅ implemented | Exported as a Rust constant module. |
| `kAXUnknownOrientationValue` | ✅ implemented | Exported as a Rust constant module. |
| `kAXAscendingSortDirectionValue` | ✅ implemented | Exported as a Rust constant module. |
| `kAXDescendingSortDirectionValue` | ✅ implemented | Exported as a Rust constant module. |
| `kAXUnknownSortDirectionValue` | ✅ implemented | Exported as a Rust constant module. |

