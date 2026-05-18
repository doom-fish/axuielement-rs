//! Accessibility attribute, role, subrole, and value constants.

/// `ApplicationServices` AX attribute string constants from `AXUIElement.h`.
pub mod attributes {
    /// Mirrors the `ApplicationServices` `kAXRoleAttribute` constant.
    pub const AX_ROLE_ATTRIBUTE: &str = "AXRole";
    /// Mirrors the `ApplicationServices` `kAXSubroleAttribute` constant.
    pub const AX_SUBROLE_ATTRIBUTE: &str = "AXSubrole";
    /// Mirrors the `ApplicationServices` `kAXRoleDescriptionAttribute` constant.
    pub const AX_ROLE_DESCRIPTION_ATTRIBUTE: &str = "AXRoleDescription";
    /// Mirrors the `ApplicationServices` `kAXHelpAttribute` constant.
    pub const AX_HELP_ATTRIBUTE: &str = "AXHelp";
    /// Mirrors the `ApplicationServices` `kAXTitleAttribute` constant.
    pub const AX_TITLE_ATTRIBUTE: &str = "AXTitle";
    /// Mirrors the `ApplicationServices` `kAXValueAttribute` constant.
    pub const AX_VALUE_ATTRIBUTE: &str = "AXValue";
    /// Mirrors the `ApplicationServices` `kAXValueDescriptionAttribute` constant.
    pub const AX_VALUE_DESCRIPTION_ATTRIBUTE: &str = "AXValueDescription";
    /// Mirrors the `ApplicationServices` `kAXMinValueAttribute` constant.
    pub const AX_MIN_VALUE_ATTRIBUTE: &str = "AXMinValue";
    /// Mirrors the `ApplicationServices` `kAXMaxValueAttribute` constant.
    pub const AX_MAX_VALUE_ATTRIBUTE: &str = "AXMaxValue";
    /// Mirrors the `ApplicationServices` `kAXValueIncrementAttribute` constant.
    pub const AX_VALUE_INCREMENT_ATTRIBUTE: &str = "AXValueIncrement";
    /// Mirrors the `ApplicationServices` `kAXAllowedValuesAttribute` constant.
    pub const AX_ALLOWED_VALUES_ATTRIBUTE: &str = "AXAllowedValues";
    /// Mirrors the `ApplicationServices` `kAXPlaceholderValueAttribute` constant.
    pub const AX_PLACEHOLDER_VALUE_ATTRIBUTE: &str = "AXPlaceholderValue";
    /// Mirrors the `ApplicationServices` `kAXEnabledAttribute` constant.
    pub const AX_ENABLED_ATTRIBUTE: &str = "AXEnabled";
    /// Mirrors the `ApplicationServices` `kAXElementBusyAttribute` constant.
    pub const AX_ELEMENT_BUSY_ATTRIBUTE: &str = "AXElementBusy";
    /// Mirrors the `ApplicationServices` `kAXFocusedAttribute` constant.
    pub const AX_FOCUSED_ATTRIBUTE: &str = "AXFocused";
    /// Mirrors the `ApplicationServices` `kAXParentAttribute` constant.
    pub const AX_PARENT_ATTRIBUTE: &str = "AXParent";
    /// Mirrors the `ApplicationServices` `kAXChildrenAttribute` constant.
    pub const AX_CHILDREN_ATTRIBUTE: &str = "AXChildren";
    /// Mirrors the `ApplicationServices` `kAXSelectedChildrenAttribute` constant.
    pub const AX_SELECTED_CHILDREN_ATTRIBUTE: &str = "AXSelectedChildren";
    /// Mirrors the `ApplicationServices` `kAXVisibleChildrenAttribute` constant.
    pub const AX_VISIBLE_CHILDREN_ATTRIBUTE: &str = "AXVisibleChildren";
    /// Mirrors the `ApplicationServices` `kAXWindowAttribute` constant.
    pub const AX_WINDOW_ATTRIBUTE: &str = "AXWindow";
    /// Mirrors the `ApplicationServices` `kAXTopLevelUIElementAttribute` constant.
    pub const AX_TOP_LEVEL_UI_ELEMENT_ATTRIBUTE: &str = "AXTopLevelUIElement";
    /// Mirrors the `ApplicationServices` `kAXPositionAttribute` constant.
    pub const AX_POSITION_ATTRIBUTE: &str = "AXPosition";
    /// Mirrors the `ApplicationServices` `kAXSizeAttribute` constant.
    pub const AX_SIZE_ATTRIBUTE: &str = "AXSize";
    /// Mirrors the `ApplicationServices` `kAXOrientationAttribute` constant.
    pub const AX_ORIENTATION_ATTRIBUTE: &str = "AXOrientation";
    /// Mirrors the `ApplicationServices` `kAXDescriptionAttribute` constant.
    pub const AX_DESCRIPTION_ATTRIBUTE: &str = "AXDescription";
    /// Mirrors the `ApplicationServices` `kAXDescriptionAttribute` constant.
    pub const AX_DESCRIPTION: &str = "AXDescription";
    /// Mirrors the `ApplicationServices` `kAXSelectedTextAttribute` constant.
    pub const AX_SELECTED_TEXT_ATTRIBUTE: &str = "AXSelectedText";
    /// Mirrors the `ApplicationServices` `kAXSelectedTextRangeAttribute` constant.
    pub const AX_SELECTED_TEXT_RANGE_ATTRIBUTE: &str = "AXSelectedTextRange";
    /// Mirrors the `ApplicationServices` `kAXSelectedTextRangesAttribute` constant.
    pub const AX_SELECTED_TEXT_RANGES_ATTRIBUTE: &str = "AXSelectedTextRanges";
    /// Mirrors the `ApplicationServices` `kAXVisibleCharacterRangeAttribute` constant.
    pub const AX_VISIBLE_CHARACTER_RANGE_ATTRIBUTE: &str = "AXVisibleCharacterRange";
    /// Mirrors the `ApplicationServices` `kAXNumberOfCharactersAttribute` constant.
    pub const AX_NUMBER_OF_CHARACTERS_ATTRIBUTE: &str = "AXNumberOfCharacters";
    /// Mirrors the `ApplicationServices` `kAXSharedTextUIElementsAttribute` constant.
    pub const AX_SHARED_TEXT_UI_ELEMENTS_ATTRIBUTE: &str = "AXSharedTextUIElements";
    /// Mirrors the `ApplicationServices` `kAXSharedCharacterRangeAttribute` constant.
    pub const AX_SHARED_CHARACTER_RANGE_ATTRIBUTE: &str = "AXSharedCharacterRange";
    /// Mirrors the `ApplicationServices` `kAXSharedFocusElementsAttribute` constant.
    pub const AX_SHARED_FOCUS_ELEMENTS_ATTRIBUTE: &str = "AXSharedFocusElements";
    /// Mirrors the `ApplicationServices` `kAXInsertionPointLineNumberAttribute` constant.
    pub const AX_INSERTION_POINT_LINE_NUMBER_ATTRIBUTE: &str = "AXInsertionPointLineNumber";
    /// Mirrors the `ApplicationServices` `kAXMainAttribute` constant.
    pub const AX_MAIN_ATTRIBUTE: &str = "AXMain";
    /// Mirrors the `ApplicationServices` `kAXMinimizedAttribute` constant.
    pub const AX_MINIMIZED_ATTRIBUTE: &str = "AXMinimized";
    /// Mirrors the `ApplicationServices` `kAXCloseButtonAttribute` constant.
    pub const AX_CLOSE_BUTTON_ATTRIBUTE: &str = "AXCloseButton";
    /// Mirrors the `ApplicationServices` `kAXZoomButtonAttribute` constant.
    pub const AX_ZOOM_BUTTON_ATTRIBUTE: &str = "AXZoomButton";
    /// Mirrors the `ApplicationServices` `kAXMinimizeButtonAttribute` constant.
    pub const AX_MINIMIZE_BUTTON_ATTRIBUTE: &str = "AXMinimizeButton";
    /// Mirrors the `ApplicationServices` `kAXToolbarButtonAttribute` constant.
    pub const AX_TOOLBAR_BUTTON_ATTRIBUTE: &str = "AXToolbarButton";
    /// Mirrors the `ApplicationServices` `kAXFullScreenButtonAttribute` constant.
    pub const AX_FULL_SCREEN_BUTTON_ATTRIBUTE: &str = "AXFullScreenButton";
    /// Mirrors the `ApplicationServices` `kAXProxyAttribute` constant.
    pub const AX_PROXY_ATTRIBUTE: &str = "AXProxy";
    /// Mirrors the `ApplicationServices` `kAXGrowAreaAttribute` constant.
    pub const AX_GROW_AREA_ATTRIBUTE: &str = "AXGrowArea";
    /// Mirrors the `ApplicationServices` `kAXModalAttribute` constant.
    pub const AX_MODAL_ATTRIBUTE: &str = "AXModal";
    /// Mirrors the `ApplicationServices` `kAXDefaultButtonAttribute` constant.
    pub const AX_DEFAULT_BUTTON_ATTRIBUTE: &str = "AXDefaultButton";
    /// Mirrors the `ApplicationServices` `kAXCancelButtonAttribute` constant.
    pub const AX_CANCEL_BUTTON_ATTRIBUTE: &str = "AXCancelButton";
    /// Mirrors the `ApplicationServices` `kAXMenuItemCmdCharAttribute` constant.
    pub const AX_MENU_ITEM_CMD_CHAR_ATTRIBUTE: &str = "AXMenuItemCmdChar";
    /// Mirrors the `ApplicationServices` `kAXMenuItemCmdVirtualKeyAttribute` constant.
    pub const AX_MENU_ITEM_CMD_VIRTUAL_KEY_ATTRIBUTE: &str = "AXMenuItemCmdVirtualKey";
    /// Mirrors the `ApplicationServices` `kAXMenuItemCmdGlyphAttribute` constant.
    pub const AX_MENU_ITEM_CMD_GLYPH_ATTRIBUTE: &str = "AXMenuItemCmdGlyph";
    /// Mirrors the `ApplicationServices` `kAXMenuItemCmdModifiersAttribute` constant.
    pub const AX_MENU_ITEM_CMD_MODIFIERS_ATTRIBUTE: &str = "AXMenuItemCmdModifiers";
    /// Mirrors the `ApplicationServices` `kAXMenuItemMarkCharAttribute` constant.
    pub const AX_MENU_ITEM_MARK_CHAR_ATTRIBUTE: &str = "AXMenuItemMarkChar";
    /// Mirrors the `ApplicationServices` `kAXMenuItemPrimaryUIElementAttribute` constant.
    pub const AX_MENU_ITEM_PRIMARY_UI_ELEMENT_ATTRIBUTE: &str = "AXMenuItemPrimaryUIElement";
    /// Mirrors the `ApplicationServices` `kAXMenuBarAttribute` constant.
    pub const AX_MENU_BAR_ATTRIBUTE: &str = "AXMenuBar";
    /// Mirrors the `ApplicationServices` `kAXWindowsAttribute` constant.
    pub const AX_WINDOWS_ATTRIBUTE: &str = "AXWindows";
    /// Mirrors the `ApplicationServices` `kAXFrontmostAttribute` constant.
    pub const AX_FRONTMOST_ATTRIBUTE: &str = "AXFrontmost";
    /// Mirrors the `ApplicationServices` `kAXHiddenAttribute` constant.
    pub const AX_HIDDEN_ATTRIBUTE: &str = "AXHidden";
    /// Mirrors the `ApplicationServices` `kAXMainWindowAttribute` constant.
    pub const AX_MAIN_WINDOW_ATTRIBUTE: &str = "AXMainWindow";
    /// Mirrors the `ApplicationServices` `kAXFocusedWindowAttribute` constant.
    pub const AX_FOCUSED_WINDOW_ATTRIBUTE: &str = "AXFocusedWindow";
    /// Mirrors the `ApplicationServices` `kAXFocusedUIElementAttribute` constant.
    pub const AX_FOCUSED_UI_ELEMENT_ATTRIBUTE: &str = "AXFocusedUIElement";
    /// Mirrors the `ApplicationServices` `kAXExtrasMenuBarAttribute` constant.
    pub const AX_EXTRAS_MENU_BAR_ATTRIBUTE: &str = "AXExtrasMenuBar";
    /// Mirrors the `ApplicationServices` `kAXHeaderAttribute` constant.
    pub const AX_HEADER_ATTRIBUTE: &str = "AXHeader";
    /// Mirrors the `ApplicationServices` `kAXEditedAttribute` constant.
    pub const AX_EDITED_ATTRIBUTE: &str = "AXEdited";
    /// Mirrors the `ApplicationServices` `kAXValueWrapsAttribute` constant.
    pub const AX_VALUE_WRAPS_ATTRIBUTE: &str = "AXValueWraps";
    /// Mirrors the `ApplicationServices` `kAXTabsAttribute` constant.
    pub const AX_TABS_ATTRIBUTE: &str = "AXTabs";
    /// Mirrors the `ApplicationServices` `kAXTitleUIElementAttribute` constant.
    pub const AX_TITLE_UI_ELEMENT_ATTRIBUTE: &str = "AXTitleUIElement";
    /// Mirrors the `ApplicationServices` `kAXHorizontalScrollBarAttribute` constant.
    pub const AX_HORIZONTAL_SCROLL_BAR_ATTRIBUTE: &str = "AXHorizontalScrollBar";
    /// Mirrors the `ApplicationServices` `kAXVerticalScrollBarAttribute` constant.
    pub const AX_VERTICAL_SCROLL_BAR_ATTRIBUTE: &str = "AXVerticalScrollBar";
    /// Mirrors the `ApplicationServices` `kAXOverflowButtonAttribute` constant.
    pub const AX_OVERFLOW_BUTTON_ATTRIBUTE: &str = "AXOverflowButton";
    /// Mirrors the `ApplicationServices` `kAXFilenameAttribute` constant.
    pub const AX_FILENAME_ATTRIBUTE: &str = "AXFilename";
    /// Mirrors the `ApplicationServices` `kAXExpandedAttribute` constant.
    pub const AX_EXPANDED_ATTRIBUTE: &str = "AXExpanded";
    /// Mirrors the `ApplicationServices` `kAXSelectedAttribute` constant.
    pub const AX_SELECTED_ATTRIBUTE: &str = "AXSelected";
    /// Mirrors the `ApplicationServices` `kAXSplittersAttribute` constant.
    pub const AX_SPLITTERS_ATTRIBUTE: &str = "AXSplitters";
    /// Mirrors the `ApplicationServices` `kAXNextContentsAttribute` constant.
    pub const AX_NEXT_CONTENTS_ATTRIBUTE: &str = "AXNextContents";
    /// Mirrors the `ApplicationServices` `kAXDocumentAttribute` constant.
    pub const AX_DOCUMENT_ATTRIBUTE: &str = "AXDocument";
    /// Mirrors the `ApplicationServices` `kAXDecrementButtonAttribute` constant.
    pub const AX_DECREMENT_BUTTON_ATTRIBUTE: &str = "AXDecrementButton";
    /// Mirrors the `ApplicationServices` `kAXIncrementButtonAttribute` constant.
    pub const AX_INCREMENT_BUTTON_ATTRIBUTE: &str = "AXIncrementButton";
    /// Mirrors the `ApplicationServices` `kAXPreviousContentsAttribute` constant.
    pub const AX_PREVIOUS_CONTENTS_ATTRIBUTE: &str = "AXPreviousContents";
    /// Mirrors the `ApplicationServices` `kAXContentsAttribute` constant.
    pub const AX_CONTENTS_ATTRIBUTE: &str = "AXContents";
    /// Mirrors the `ApplicationServices` `kAXIncrementorAttribute` constant.
    pub const AX_INCREMENTOR_ATTRIBUTE: &str = "AXIncrementor";
    /// Mirrors the `ApplicationServices` `kAXHourFieldAttribute` constant.
    pub const AX_HOUR_FIELD_ATTRIBUTE: &str = "AXHourField";
    /// Mirrors the `ApplicationServices` `kAXMinuteFieldAttribute` constant.
    pub const AX_MINUTE_FIELD_ATTRIBUTE: &str = "AXMinuteField";
    /// Mirrors the `ApplicationServices` `kAXSecondFieldAttribute` constant.
    pub const AX_SECOND_FIELD_ATTRIBUTE: &str = "AXSecondField";
    /// Mirrors the `ApplicationServices` `kAXAMPMFieldAttribute` constant.
    pub const AXAMPM_FIELD_ATTRIBUTE: &str = "AXAMPMField";
    /// Mirrors the `ApplicationServices` `kAXDayFieldAttribute` constant.
    pub const AX_DAY_FIELD_ATTRIBUTE: &str = "AXDayField";
    /// Mirrors the `ApplicationServices` `kAXMonthFieldAttribute` constant.
    pub const AX_MONTH_FIELD_ATTRIBUTE: &str = "AXMonthField";
    /// Mirrors the `ApplicationServices` `kAXYearFieldAttribute` constant.
    pub const AX_YEAR_FIELD_ATTRIBUTE: &str = "AXYearField";
    /// Mirrors the `ApplicationServices` `kAXColumnTitleAttribute` constant.
    pub const AX_COLUMN_TITLE_ATTRIBUTE: &str = "AXColumnTitles";
    /// Mirrors the `ApplicationServices` `kAXURLAttribute` constant.
    pub const AXURL_ATTRIBUTE: &str = "AXURL";
    /// Mirrors the `ApplicationServices` `kAXLabelUIElementsAttribute` constant.
    pub const AX_LABEL_UI_ELEMENTS_ATTRIBUTE: &str = "AXLabelUIElements";
    /// Mirrors the `ApplicationServices` `kAXLabelValueAttribute` constant.
    pub const AX_LABEL_VALUE_ATTRIBUTE: &str = "AXLabelValue";
    /// Mirrors the `ApplicationServices` `kAXShownMenuUIElementAttribute` constant.
    pub const AX_SHOWN_MENU_UI_ELEMENT_ATTRIBUTE: &str = "AXShownMenuUIElement";
    /// Mirrors the `ApplicationServices` `kAXServesAsTitleForUIElementsAttribute` constant.
    pub const AX_SERVES_AS_TITLE_FOR_UI_ELEMENTS_ATTRIBUTE: &str = "AXServesAsTitleForUIElements";
    /// Mirrors the `ApplicationServices` `kAXLinkedUIElementsAttribute` constant.
    pub const AX_LINKED_UI_ELEMENTS_ATTRIBUTE: &str = "AXLinkedUIElements";
    /// Mirrors the `ApplicationServices` `kAXRowsAttribute` constant.
    pub const AX_ROWS_ATTRIBUTE: &str = "AXRows";
    /// Mirrors the `ApplicationServices` `kAXVisibleRowsAttribute` constant.
    pub const AX_VISIBLE_ROWS_ATTRIBUTE: &str = "AXVisibleRows";
    /// Mirrors the `ApplicationServices` `kAXSelectedRowsAttribute` constant.
    pub const AX_SELECTED_ROWS_ATTRIBUTE: &str = "AXSelectedRows";
    /// Mirrors the `ApplicationServices` `kAXColumnsAttribute` constant.
    pub const AX_COLUMNS_ATTRIBUTE: &str = "AXColumns";
    /// Mirrors the `ApplicationServices` `kAXVisibleColumnsAttribute` constant.
    pub const AX_VISIBLE_COLUMNS_ATTRIBUTE: &str = "AXVisibleColumns";
    /// Mirrors the `ApplicationServices` `kAXSelectedColumnsAttribute` constant.
    pub const AX_SELECTED_COLUMNS_ATTRIBUTE: &str = "AXSelectedColumns";
    /// Mirrors the `ApplicationServices` `kAXSortDirectionAttribute` constant.
    pub const AX_SORT_DIRECTION_ATTRIBUTE: &str = "AXSortDirection";
    /// Mirrors the `ApplicationServices` `kAXIndexAttribute` constant.
    pub const AX_INDEX_ATTRIBUTE: &str = "AXIndex";
    /// Mirrors the `ApplicationServices` `kAXDisclosingAttribute` constant.
    pub const AX_DISCLOSING_ATTRIBUTE: &str = "AXDisclosing";
    /// Mirrors the `ApplicationServices` `kAXDisclosedRowsAttribute` constant.
    pub const AX_DISCLOSED_ROWS_ATTRIBUTE: &str = "AXDisclosedRows";
    /// Mirrors the `ApplicationServices` `kAXDisclosedByRowAttribute` constant.
    pub const AX_DISCLOSED_BY_ROW_ATTRIBUTE: &str = "AXDisclosedByRow";
    /// Mirrors the `ApplicationServices` `kAXDisclosureLevelAttribute` constant.
    pub const AX_DISCLOSURE_LEVEL_ATTRIBUTE: &str = "AXDisclosureLevel";
    /// Mirrors the `ApplicationServices` `kAXMatteHoleAttribute` constant.
    pub const AX_MATTE_HOLE_ATTRIBUTE: &str = "AXMatteHole";
    /// Mirrors the `ApplicationServices` `kAXMatteContentUIElementAttribute` constant.
    pub const AX_MATTE_CONTENT_UI_ELEMENT_ATTRIBUTE: &str = "AXMatteContentUIElement";
    /// Mirrors the `ApplicationServices` `kAXMarkerUIElementsAttribute` constant.
    pub const AX_MARKER_UI_ELEMENTS_ATTRIBUTE: &str = "AXMarkerUIElements";
    /// Mirrors the `ApplicationServices` `kAXUnitsAttribute` constant.
    pub const AX_UNITS_ATTRIBUTE: &str = "AXUnits";
    /// Mirrors the `ApplicationServices` `kAXUnitDescriptionAttribute` constant.
    pub const AX_UNIT_DESCRIPTION_ATTRIBUTE: &str = "AXUnitDescription";
    /// Mirrors the `ApplicationServices` `kAXMarkerTypeAttribute` constant.
    pub const AX_MARKER_TYPE_ATTRIBUTE: &str = "AXMarkerType";
    /// Mirrors the `ApplicationServices` `kAXMarkerTypeDescriptionAttribute` constant.
    pub const AX_MARKER_TYPE_DESCRIPTION_ATTRIBUTE: &str = "AXMarkerTypeDescription";
    /// Mirrors the `ApplicationServices` `kAXIsApplicationRunningAttribute` constant.
    pub const AX_IS_APPLICATION_RUNNING_ATTRIBUTE: &str = "AXIsApplicationRunning";
    /// Mirrors the `ApplicationServices` `kAXSearchButtonAttribute` constant.
    pub const AX_SEARCH_BUTTON_ATTRIBUTE: &str = "AXSearchButton";
    /// Mirrors the `ApplicationServices` `kAXClearButtonAttribute` constant.
    pub const AX_CLEAR_BUTTON_ATTRIBUTE: &str = "AXClearButton";
    /// Mirrors the `ApplicationServices` `kAXFocusedApplicationAttribute` constant.
    pub const AX_FOCUSED_APPLICATION_ATTRIBUTE: &str = "AXFocusedApplication";
    /// Mirrors the `ApplicationServices` `kAXRowCountAttribute` constant.
    pub const AX_ROW_COUNT_ATTRIBUTE: &str = "AXRowCount";
    /// Mirrors the `ApplicationServices` `kAXColumnCountAttribute` constant.
    pub const AX_COLUMN_COUNT_ATTRIBUTE: &str = "AXColumnCount";
    /// Mirrors the `ApplicationServices` `kAXOrderedByRowAttribute` constant.
    pub const AX_ORDERED_BY_ROW_ATTRIBUTE: &str = "AXOrderedByRow";
    /// Mirrors the `ApplicationServices` `kAXWarningValueAttribute` constant.
    pub const AX_WARNING_VALUE_ATTRIBUTE: &str = "AXWarningValue";
    /// Mirrors the `ApplicationServices` `kAXCriticalValueAttribute` constant.
    pub const AX_CRITICAL_VALUE_ATTRIBUTE: &str = "AXCriticalValue";
    /// Mirrors the `ApplicationServices` `kAXSelectedCellsAttribute` constant.
    pub const AX_SELECTED_CELLS_ATTRIBUTE: &str = "AXSelectedCells";
    /// Mirrors the `ApplicationServices` `kAXVisibleCellsAttribute` constant.
    pub const AX_VISIBLE_CELLS_ATTRIBUTE: &str = "AXVisibleCells";
    /// Mirrors the `ApplicationServices` `kAXRowHeaderUIElementsAttribute` constant.
    pub const AX_ROW_HEADER_UI_ELEMENTS_ATTRIBUTE: &str = "AXRowHeaderUIElements";
    /// Mirrors the `ApplicationServices` `kAXColumnHeaderUIElementsAttribute` constant.
    pub const AX_COLUMN_HEADER_UI_ELEMENTS_ATTRIBUTE: &str = "AXColumnHeaderUIElements";
    /// Mirrors the `ApplicationServices` `kAXRowIndexRangeAttribute` constant.
    pub const AX_ROW_INDEX_RANGE_ATTRIBUTE: &str = "AXRowIndexRange";
    /// Mirrors the `ApplicationServices` `kAXColumnIndexRangeAttribute` constant.
    pub const AX_COLUMN_INDEX_RANGE_ATTRIBUTE: &str = "AXColumnIndexRange";
    /// Mirrors the `ApplicationServices` `kAXHorizontalUnitsAttribute` constant.
    pub const AX_HORIZONTAL_UNITS_ATTRIBUTE: &str = "AXHorizontalUnits";
    /// Mirrors the `ApplicationServices` `kAXVerticalUnitsAttribute` constant.
    pub const AX_VERTICAL_UNITS_ATTRIBUTE: &str = "AXVerticalUnits";
    /// Mirrors the `ApplicationServices` `kAXHorizontalUnitDescriptionAttribute` constant.
    pub const AX_HORIZONTAL_UNIT_DESCRIPTION_ATTRIBUTE: &str = "AXHorizontalUnitDescription";
    /// Mirrors the `ApplicationServices` `kAXVerticalUnitDescriptionAttribute` constant.
    pub const AX_VERTICAL_UNIT_DESCRIPTION_ATTRIBUTE: &str = "AXVerticalUnitDescription";
    /// Mirrors the `ApplicationServices` `kAXHandlesAttribute` constant.
    pub const AX_HANDLES_ATTRIBUTE: &str = "AXHandles";
    /// Mirrors the `ApplicationServices` `kAXTextAttribute` constant.
    pub const AX_TEXT_ATTRIBUTE: &str = "AXText";
    /// Mirrors the `ApplicationServices` `kAXVisibleTextAttribute` constant.
    pub const AX_VISIBLE_TEXT_ATTRIBUTE: &str = "AXVisibleText";
    /// Mirrors the `ApplicationServices` `kAXIsEditableAttribute` constant.
    pub const AX_IS_EDITABLE_ATTRIBUTE: &str = "AXIsEditable";
    /// Mirrors the `ApplicationServices` `kAXColumnTitlesAttribute` constant.
    pub const AX_COLUMN_TITLES_ATTRIBUTE: &str = "AXColumnTitles";
    /// Mirrors the `ApplicationServices` `kAXIDentifierAttribute` constant.
    pub const AX_IDENTIFIER_ATTRIBUTE: &str = "AXIdentifier";
    /// Mirrors the `ApplicationServices` `kAXAlternateUIVisibleAttribute` constant.
    pub const AX_ALTERNATE_UI_VISIBLE_ATTRIBUTE: &str = "AXAlternateUIVisible";

    /// Collects the `ApplicationServices` AX attribute constants exposed by this module.
    pub const ALL_ATTRIBUTES: &[&str] = &[
        AX_ROLE_ATTRIBUTE,
        AX_SUBROLE_ATTRIBUTE,
        AX_ROLE_DESCRIPTION_ATTRIBUTE,
        AX_HELP_ATTRIBUTE,
        AX_TITLE_ATTRIBUTE,
        AX_VALUE_ATTRIBUTE,
        AX_VALUE_DESCRIPTION_ATTRIBUTE,
        AX_MIN_VALUE_ATTRIBUTE,
        AX_MAX_VALUE_ATTRIBUTE,
        AX_VALUE_INCREMENT_ATTRIBUTE,
        AX_ALLOWED_VALUES_ATTRIBUTE,
        AX_PLACEHOLDER_VALUE_ATTRIBUTE,
        AX_ENABLED_ATTRIBUTE,
        AX_ELEMENT_BUSY_ATTRIBUTE,
        AX_FOCUSED_ATTRIBUTE,
        AX_PARENT_ATTRIBUTE,
        AX_CHILDREN_ATTRIBUTE,
        AX_SELECTED_CHILDREN_ATTRIBUTE,
        AX_VISIBLE_CHILDREN_ATTRIBUTE,
        AX_WINDOW_ATTRIBUTE,
        AX_TOP_LEVEL_UI_ELEMENT_ATTRIBUTE,
        AX_POSITION_ATTRIBUTE,
        AX_SIZE_ATTRIBUTE,
        AX_ORIENTATION_ATTRIBUTE,
        AX_DESCRIPTION_ATTRIBUTE,
        AX_DESCRIPTION,
        AX_SELECTED_TEXT_ATTRIBUTE,
        AX_SELECTED_TEXT_RANGE_ATTRIBUTE,
        AX_SELECTED_TEXT_RANGES_ATTRIBUTE,
        AX_VISIBLE_CHARACTER_RANGE_ATTRIBUTE,
        AX_NUMBER_OF_CHARACTERS_ATTRIBUTE,
        AX_SHARED_TEXT_UI_ELEMENTS_ATTRIBUTE,
        AX_SHARED_CHARACTER_RANGE_ATTRIBUTE,
        AX_SHARED_FOCUS_ELEMENTS_ATTRIBUTE,
        AX_INSERTION_POINT_LINE_NUMBER_ATTRIBUTE,
        AX_MAIN_ATTRIBUTE,
        AX_MINIMIZED_ATTRIBUTE,
        AX_CLOSE_BUTTON_ATTRIBUTE,
        AX_ZOOM_BUTTON_ATTRIBUTE,
        AX_MINIMIZE_BUTTON_ATTRIBUTE,
        AX_TOOLBAR_BUTTON_ATTRIBUTE,
        AX_FULL_SCREEN_BUTTON_ATTRIBUTE,
        AX_PROXY_ATTRIBUTE,
        AX_GROW_AREA_ATTRIBUTE,
        AX_MODAL_ATTRIBUTE,
        AX_DEFAULT_BUTTON_ATTRIBUTE,
        AX_CANCEL_BUTTON_ATTRIBUTE,
        AX_MENU_ITEM_CMD_CHAR_ATTRIBUTE,
        AX_MENU_ITEM_CMD_VIRTUAL_KEY_ATTRIBUTE,
        AX_MENU_ITEM_CMD_GLYPH_ATTRIBUTE,
        AX_MENU_ITEM_CMD_MODIFIERS_ATTRIBUTE,
        AX_MENU_ITEM_MARK_CHAR_ATTRIBUTE,
        AX_MENU_ITEM_PRIMARY_UI_ELEMENT_ATTRIBUTE,
        AX_MENU_BAR_ATTRIBUTE,
        AX_WINDOWS_ATTRIBUTE,
        AX_FRONTMOST_ATTRIBUTE,
        AX_HIDDEN_ATTRIBUTE,
        AX_MAIN_WINDOW_ATTRIBUTE,
        AX_FOCUSED_WINDOW_ATTRIBUTE,
        AX_FOCUSED_UI_ELEMENT_ATTRIBUTE,
        AX_EXTRAS_MENU_BAR_ATTRIBUTE,
        AX_HEADER_ATTRIBUTE,
        AX_EDITED_ATTRIBUTE,
        AX_VALUE_WRAPS_ATTRIBUTE,
        AX_TABS_ATTRIBUTE,
        AX_TITLE_UI_ELEMENT_ATTRIBUTE,
        AX_HORIZONTAL_SCROLL_BAR_ATTRIBUTE,
        AX_VERTICAL_SCROLL_BAR_ATTRIBUTE,
        AX_OVERFLOW_BUTTON_ATTRIBUTE,
        AX_FILENAME_ATTRIBUTE,
        AX_EXPANDED_ATTRIBUTE,
        AX_SELECTED_ATTRIBUTE,
        AX_SPLITTERS_ATTRIBUTE,
        AX_NEXT_CONTENTS_ATTRIBUTE,
        AX_DOCUMENT_ATTRIBUTE,
        AX_DECREMENT_BUTTON_ATTRIBUTE,
        AX_INCREMENT_BUTTON_ATTRIBUTE,
        AX_PREVIOUS_CONTENTS_ATTRIBUTE,
        AX_CONTENTS_ATTRIBUTE,
        AX_INCREMENTOR_ATTRIBUTE,
        AX_HOUR_FIELD_ATTRIBUTE,
        AX_MINUTE_FIELD_ATTRIBUTE,
        AX_SECOND_FIELD_ATTRIBUTE,
        AXAMPM_FIELD_ATTRIBUTE,
        AX_DAY_FIELD_ATTRIBUTE,
        AX_MONTH_FIELD_ATTRIBUTE,
        AX_YEAR_FIELD_ATTRIBUTE,
        AX_COLUMN_TITLE_ATTRIBUTE,
        AXURL_ATTRIBUTE,
        AX_LABEL_UI_ELEMENTS_ATTRIBUTE,
        AX_LABEL_VALUE_ATTRIBUTE,
        AX_SHOWN_MENU_UI_ELEMENT_ATTRIBUTE,
        AX_SERVES_AS_TITLE_FOR_UI_ELEMENTS_ATTRIBUTE,
        AX_LINKED_UI_ELEMENTS_ATTRIBUTE,
        AX_ROWS_ATTRIBUTE,
        AX_VISIBLE_ROWS_ATTRIBUTE,
        AX_SELECTED_ROWS_ATTRIBUTE,
        AX_COLUMNS_ATTRIBUTE,
        AX_VISIBLE_COLUMNS_ATTRIBUTE,
        AX_SELECTED_COLUMNS_ATTRIBUTE,
        AX_SORT_DIRECTION_ATTRIBUTE,
        AX_INDEX_ATTRIBUTE,
        AX_DISCLOSING_ATTRIBUTE,
        AX_DISCLOSED_ROWS_ATTRIBUTE,
        AX_DISCLOSED_BY_ROW_ATTRIBUTE,
        AX_DISCLOSURE_LEVEL_ATTRIBUTE,
        AX_MATTE_HOLE_ATTRIBUTE,
        AX_MATTE_CONTENT_UI_ELEMENT_ATTRIBUTE,
        AX_MARKER_UI_ELEMENTS_ATTRIBUTE,
        AX_UNITS_ATTRIBUTE,
        AX_UNIT_DESCRIPTION_ATTRIBUTE,
        AX_MARKER_TYPE_ATTRIBUTE,
        AX_MARKER_TYPE_DESCRIPTION_ATTRIBUTE,
        AX_IS_APPLICATION_RUNNING_ATTRIBUTE,
        AX_SEARCH_BUTTON_ATTRIBUTE,
        AX_CLEAR_BUTTON_ATTRIBUTE,
        AX_FOCUSED_APPLICATION_ATTRIBUTE,
        AX_ROW_COUNT_ATTRIBUTE,
        AX_COLUMN_COUNT_ATTRIBUTE,
        AX_ORDERED_BY_ROW_ATTRIBUTE,
        AX_WARNING_VALUE_ATTRIBUTE,
        AX_CRITICAL_VALUE_ATTRIBUTE,
        AX_SELECTED_CELLS_ATTRIBUTE,
        AX_VISIBLE_CELLS_ATTRIBUTE,
        AX_ROW_HEADER_UI_ELEMENTS_ATTRIBUTE,
        AX_COLUMN_HEADER_UI_ELEMENTS_ATTRIBUTE,
        AX_ROW_INDEX_RANGE_ATTRIBUTE,
        AX_COLUMN_INDEX_RANGE_ATTRIBUTE,
        AX_HORIZONTAL_UNITS_ATTRIBUTE,
        AX_VERTICAL_UNITS_ATTRIBUTE,
        AX_HORIZONTAL_UNIT_DESCRIPTION_ATTRIBUTE,
        AX_VERTICAL_UNIT_DESCRIPTION_ATTRIBUTE,
        AX_HANDLES_ATTRIBUTE,
        AX_TEXT_ATTRIBUTE,
        AX_VISIBLE_TEXT_ATTRIBUTE,
        AX_IS_EDITABLE_ATTRIBUTE,
        AX_COLUMN_TITLES_ATTRIBUTE,
        AX_IDENTIFIER_ATTRIBUTE,
        AX_ALTERNATE_UI_VISIBLE_ATTRIBUTE,
    ];
}

/// `ApplicationServices` AX parameterized-attribute string constants from `AXUIElement.h`.
pub mod parameterized {
    /// Mirrors the `ApplicationServices` `kAXLineForIndexParameterizedAttribute` constant.
    pub const AX_LINE_FOR_INDEX_PARAMETERIZED_ATTRIBUTE: &str = "AXLineForIndex";
    /// Mirrors the `ApplicationServices` `kAXRangeForLineParameterizedAttribute` constant.
    pub const AX_RANGE_FOR_LINE_PARAMETERIZED_ATTRIBUTE: &str = "AXRangeForLine";
    /// Mirrors the `ApplicationServices` `kAXStringForRangeParameterizedAttribute` constant.
    pub const AX_STRING_FOR_RANGE_PARAMETERIZED_ATTRIBUTE: &str = "AXStringForRange";
    /// Mirrors the `ApplicationServices` `kAXRangeForPositionParameterizedAttribute` constant.
    pub const AX_RANGE_FOR_POSITION_PARAMETERIZED_ATTRIBUTE: &str = "AXRangeForPosition";
    /// Mirrors the `ApplicationServices` `kAXRangeForIndexParameterizedAttribute` constant.
    pub const AX_RANGE_FOR_INDEX_PARAMETERIZED_ATTRIBUTE: &str = "AXRangeForIndex";
    /// Mirrors the `ApplicationServices` `kAXBoundsForRangeParameterizedAttribute` constant.
    pub const AX_BOUNDS_FOR_RANGE_PARAMETERIZED_ATTRIBUTE: &str = "AXBoundsForRange";
    /// Mirrors the `ApplicationServices` `kAXRTFForRangeParameterizedAttribute` constant.
    pub const AXRTF_FOR_RANGE_PARAMETERIZED_ATTRIBUTE: &str = "AXRTFForRange";
    /// Mirrors the `ApplicationServices` `kAXAttributedStringForRangeParameterizedAttribute` constant.
    pub const AX_ATTRIBUTED_STRING_FOR_RANGE_PARAMETERIZED_ATTRIBUTE: &str =
        "AXAttributedStringForRange";
    /// Mirrors the `ApplicationServices` `kAXStyleRangeForIndexParameterizedAttribute` constant.
    pub const AX_STYLE_RANGE_FOR_INDEX_PARAMETERIZED_ATTRIBUTE: &str = "AXStyleRangeForIndex";
    /// Mirrors the `ApplicationServices` `kAXCellForColumnAndRowParameterizedAttribute` constant.
    pub const AX_CELL_FOR_COLUMN_AND_ROW_PARAMETERIZED_ATTRIBUTE: &str = "AXCellForColumnAndRow";
    /// Mirrors the `ApplicationServices` `kAXLayoutPointForScreenPointParameterizedAttribute` constant.
    pub const AX_LAYOUT_POINT_FOR_SCREEN_POINT_PARAMETERIZED_ATTRIBUTE: &str =
        "AXLayoutPointForScreenPoint";
    /// Mirrors the `ApplicationServices` `kAXLayoutSizeForScreenSizeParameterizedAttribute` constant.
    pub const AX_LAYOUT_SIZE_FOR_SCREEN_SIZE_PARAMETERIZED_ATTRIBUTE: &str =
        "AXLayoutSizeForScreenSize";
    /// Mirrors the `ApplicationServices` `kAXScreenPointForLayoutPointParameterizedAttribute` constant.
    pub const AX_SCREEN_POINT_FOR_LAYOUT_POINT_PARAMETERIZED_ATTRIBUTE: &str =
        "AXScreenPointForLayoutPoint";
    /// Mirrors the `ApplicationServices` `kAXScreenSizeForLayoutSizeParameterizedAttribute` constant.
    pub const AX_SCREEN_SIZE_FOR_LAYOUT_SIZE_PARAMETERIZED_ATTRIBUTE: &str =
        "AXScreenSizeForLayoutSize";

    /// Collects the `ApplicationServices` AX parameterized-attribute constants exposed by this module.
    pub const ALL_PARAMETERIZED_ATTRIBUTES: &[&str] = &[
        AX_LINE_FOR_INDEX_PARAMETERIZED_ATTRIBUTE,
        AX_RANGE_FOR_LINE_PARAMETERIZED_ATTRIBUTE,
        AX_STRING_FOR_RANGE_PARAMETERIZED_ATTRIBUTE,
        AX_RANGE_FOR_POSITION_PARAMETERIZED_ATTRIBUTE,
        AX_RANGE_FOR_INDEX_PARAMETERIZED_ATTRIBUTE,
        AX_BOUNDS_FOR_RANGE_PARAMETERIZED_ATTRIBUTE,
        AXRTF_FOR_RANGE_PARAMETERIZED_ATTRIBUTE,
        AX_ATTRIBUTED_STRING_FOR_RANGE_PARAMETERIZED_ATTRIBUTE,
        AX_STYLE_RANGE_FOR_INDEX_PARAMETERIZED_ATTRIBUTE,
        AX_CELL_FOR_COLUMN_AND_ROW_PARAMETERIZED_ATTRIBUTE,
        AX_LAYOUT_POINT_FOR_SCREEN_POINT_PARAMETERIZED_ATTRIBUTE,
        AX_LAYOUT_SIZE_FOR_SCREEN_SIZE_PARAMETERIZED_ATTRIBUTE,
        AX_SCREEN_POINT_FOR_LAYOUT_POINT_PARAMETERIZED_ATTRIBUTE,
        AX_SCREEN_SIZE_FOR_LAYOUT_SIZE_PARAMETERIZED_ATTRIBUTE,
    ];
}

/// `ApplicationServices` AX role string constants from `AXUIElement.h`.
pub mod roles {
    /// Mirrors the `ApplicationServices` `kAXApplicationRole` constant.
    pub const AX_APPLICATION_ROLE: &str = "AXApplication";
    /// Mirrors the `ApplicationServices` `kAXSystemWideRole` constant.
    pub const AX_SYSTEM_WIDE_ROLE: &str = "AXSystemWide";
    /// Mirrors the `ApplicationServices` `kAXWindowRole` constant.
    pub const AX_WINDOW_ROLE: &str = "AXWindow";
    /// Mirrors the `ApplicationServices` `kAXSheetRole` constant.
    pub const AX_SHEET_ROLE: &str = "AXSheet";
    /// Mirrors the `ApplicationServices` `kAXDrawerRole` constant.
    pub const AX_DRAWER_ROLE: &str = "AXDrawer";
    /// Mirrors the `ApplicationServices` `kAXGrowAreaRole` constant.
    pub const AX_GROW_AREA_ROLE: &str = "AXGrowArea";
    /// Mirrors the `ApplicationServices` `kAXImageRole` constant.
    pub const AX_IMAGE_ROLE: &str = "AXImage";
    /// Mirrors the `ApplicationServices` `kAXUnknownRole` constant.
    pub const AX_UNKNOWN_ROLE: &str = "AXUnknown";
    /// Mirrors the `ApplicationServices` `kAXButtonRole` constant.
    pub const AX_BUTTON_ROLE: &str = "AXButton";
    /// Mirrors the `ApplicationServices` `kAXRadioButtonRole` constant.
    pub const AX_RADIO_BUTTON_ROLE: &str = "AXRadioButton";
    /// Mirrors the `ApplicationServices` `kAXCheckBoxRole` constant.
    pub const AX_CHECK_BOX_ROLE: &str = "AXCheckBox";
    /// Mirrors the `ApplicationServices` `kAXPopUpButtonRole` constant.
    pub const AX_POP_UP_BUTTON_ROLE: &str = "AXPopUpButton";
    /// Mirrors the `ApplicationServices` `kAXMenuButtonRole` constant.
    pub const AX_MENU_BUTTON_ROLE: &str = "AXMenuButton";
    /// Mirrors the `ApplicationServices` `kAXTabGroupRole` constant.
    pub const AX_TAB_GROUP_ROLE: &str = "AXTabGroup";
    /// Mirrors the `ApplicationServices` `kAXTableRole` constant.
    pub const AX_TABLE_ROLE: &str = "AXTable";
    /// Mirrors the `ApplicationServices` `kAXColumnRole` constant.
    pub const AX_COLUMN_ROLE: &str = "AXColumn";
    /// Mirrors the `ApplicationServices` `kAXRowRole` constant.
    pub const AX_ROW_ROLE: &str = "AXRow";
    /// Mirrors the `ApplicationServices` `kAXOutlineRole` constant.
    pub const AX_OUTLINE_ROLE: &str = "AXOutline";
    /// Mirrors the `ApplicationServices` `kAXBrowserRole` constant.
    pub const AX_BROWSER_ROLE: &str = "AXBrowser";
    /// Mirrors the `ApplicationServices` `kAXScrollAreaRole` constant.
    pub const AX_SCROLL_AREA_ROLE: &str = "AXScrollArea";
    /// Mirrors the `ApplicationServices` `kAXScrollBarRole` constant.
    pub const AX_SCROLL_BAR_ROLE: &str = "AXScrollBar";
    /// Mirrors the `ApplicationServices` `kAXRadioGroupRole` constant.
    pub const AX_RADIO_GROUP_ROLE: &str = "AXRadioGroup";
    /// Mirrors the `ApplicationServices` `kAXListRole` constant.
    pub const AX_LIST_ROLE: &str = "AXList";
    /// Mirrors the `ApplicationServices` `kAXGroupRole` constant.
    pub const AX_GROUP_ROLE: &str = "AXGroup";
    /// Mirrors the `ApplicationServices` `kAXValueIndicatorRole` constant.
    pub const AX_VALUE_INDICATOR_ROLE: &str = "AXValueIndicator";
    /// Mirrors the `ApplicationServices` `kAXComboBoxRole` constant.
    pub const AX_COMBO_BOX_ROLE: &str = "AXComboBox";
    /// Mirrors the `ApplicationServices` `kAXSliderRole` constant.
    pub const AX_SLIDER_ROLE: &str = "AXSlider";
    /// Mirrors the `ApplicationServices` `kAXIncrementorRole` constant.
    pub const AX_INCREMENTOR_ROLE: &str = "AXIncrementor";
    /// Mirrors the `ApplicationServices` `kAXBusyIndicatorRole` constant.
    pub const AX_BUSY_INDICATOR_ROLE: &str = "AXBusyIndicator";
    /// Mirrors the `ApplicationServices` `kAXProgressIndicatorRole` constant.
    pub const AX_PROGRESS_INDICATOR_ROLE: &str = "AXProgressIndicator";
    /// Mirrors the `ApplicationServices` `kAXRelevanceIndicatorRole` constant.
    pub const AX_RELEVANCE_INDICATOR_ROLE: &str = "AXRelevanceIndicator";
    /// Mirrors the `ApplicationServices` `kAXToolbarRole` constant.
    pub const AX_TOOLBAR_ROLE: &str = "AXToolbar";
    /// Mirrors the `ApplicationServices` `kAXDisclosureTriangleRole` constant.
    pub const AX_DISCLOSURE_TRIANGLE_ROLE: &str = "AXDisclosureTriangle";
    /// Mirrors the `ApplicationServices` `kAXTextFieldRole` constant.
    pub const AX_TEXT_FIELD_ROLE: &str = "AXTextField";
    /// Mirrors the `ApplicationServices` `kAXTextAreaRole` constant.
    pub const AX_TEXT_AREA_ROLE: &str = "AXTextArea";
    /// Mirrors the `ApplicationServices` `kAXStaticTextRole` constant.
    pub const AX_STATIC_TEXT_ROLE: &str = "AXStaticText";
    /// Mirrors the `ApplicationServices` `kAXHeadingRole` constant.
    pub const AX_HEADING_ROLE: &str = "AXHeading";
    /// Mirrors the `ApplicationServices` `kAXMenuBarRole` constant.
    pub const AX_MENU_BAR_ROLE: &str = "AXMenuBar";
    /// Mirrors the `ApplicationServices` `kAXMenuBarItemRole` constant.
    pub const AX_MENU_BAR_ITEM_ROLE: &str = "AXMenuBarItem";
    /// Mirrors the `ApplicationServices` `kAXMenuRole` constant.
    pub const AX_MENU_ROLE: &str = "AXMenu";
    /// Mirrors the `ApplicationServices` `kAXMenuItemRole` constant.
    pub const AX_MENU_ITEM_ROLE: &str = "AXMenuItem";
    /// Mirrors the `ApplicationServices` `kAXSplitGroupRole` constant.
    pub const AX_SPLIT_GROUP_ROLE: &str = "AXSplitGroup";
    /// Mirrors the `ApplicationServices` `kAXSplitterRole` constant.
    pub const AX_SPLITTER_ROLE: &str = "AXSplitter";
    /// Mirrors the `ApplicationServices` `kAXColorWellRole` constant.
    pub const AX_COLOR_WELL_ROLE: &str = "AXColorWell";
    /// Mirrors the `ApplicationServices` `kAXTimeFieldRole` constant.
    pub const AX_TIME_FIELD_ROLE: &str = "AXTimeField";
    /// Mirrors the `ApplicationServices` `kAXDateFieldRole` constant.
    pub const AX_DATE_FIELD_ROLE: &str = "AXDateField";
    /// Mirrors the `ApplicationServices` `kAXHelpTagRole` constant.
    pub const AX_HELP_TAG_ROLE: &str = "AXHelpTag";
    /// Mirrors the `ApplicationServices` `kAXMatteRole` constant.
    pub const AX_MATTE_ROLE: &str = "AXMatte";
    /// Mirrors the `ApplicationServices` `kAXDockItemRole` constant.
    pub const AX_DOCK_ITEM_ROLE: &str = "AXDockItem";
    /// Mirrors the `ApplicationServices` `kAXRulerRole` constant.
    pub const AX_RULER_ROLE: &str = "AXRuler";
    /// Mirrors the `ApplicationServices` `kAXRulerMarkerRole` constant.
    pub const AX_RULER_MARKER_ROLE: &str = "AXRulerMarker";
    /// Mirrors the `ApplicationServices` `kAXGridRole` constant.
    pub const AX_GRID_ROLE: &str = "AXGrid";
    /// Mirrors the `ApplicationServices` `kAXLevelIndicatorRole` constant.
    pub const AX_LEVEL_INDICATOR_ROLE: &str = "AXLevelIndicator";
    /// Mirrors the `ApplicationServices` `kAXCellRole` constant.
    pub const AX_CELL_ROLE: &str = "AXCell";
    /// Mirrors the `ApplicationServices` `kAXLayoutAreaRole` constant.
    pub const AX_LAYOUT_AREA_ROLE: &str = "AXLayoutArea";
    /// Mirrors the `ApplicationServices` `kAXLayoutItemRole` constant.
    pub const AX_LAYOUT_ITEM_ROLE: &str = "AXLayoutItem";
    /// Mirrors the `ApplicationServices` `kAXHandleRole` constant.
    pub const AX_HANDLE_ROLE: &str = "AXHandle";
    /// Mirrors the `ApplicationServices` `kAXPopoverRole` constant.
    pub const AX_POPOVER_ROLE: &str = "AXPopover";

    /// Collects the `ApplicationServices` AX role constants exposed by this module.
    pub const ALL_ROLES: &[&str] = &[
        AX_APPLICATION_ROLE,
        AX_SYSTEM_WIDE_ROLE,
        AX_WINDOW_ROLE,
        AX_SHEET_ROLE,
        AX_DRAWER_ROLE,
        AX_GROW_AREA_ROLE,
        AX_IMAGE_ROLE,
        AX_UNKNOWN_ROLE,
        AX_BUTTON_ROLE,
        AX_RADIO_BUTTON_ROLE,
        AX_CHECK_BOX_ROLE,
        AX_POP_UP_BUTTON_ROLE,
        AX_MENU_BUTTON_ROLE,
        AX_TAB_GROUP_ROLE,
        AX_TABLE_ROLE,
        AX_COLUMN_ROLE,
        AX_ROW_ROLE,
        AX_OUTLINE_ROLE,
        AX_BROWSER_ROLE,
        AX_SCROLL_AREA_ROLE,
        AX_SCROLL_BAR_ROLE,
        AX_RADIO_GROUP_ROLE,
        AX_LIST_ROLE,
        AX_GROUP_ROLE,
        AX_VALUE_INDICATOR_ROLE,
        AX_COMBO_BOX_ROLE,
        AX_SLIDER_ROLE,
        AX_INCREMENTOR_ROLE,
        AX_BUSY_INDICATOR_ROLE,
        AX_PROGRESS_INDICATOR_ROLE,
        AX_RELEVANCE_INDICATOR_ROLE,
        AX_TOOLBAR_ROLE,
        AX_DISCLOSURE_TRIANGLE_ROLE,
        AX_TEXT_FIELD_ROLE,
        AX_TEXT_AREA_ROLE,
        AX_STATIC_TEXT_ROLE,
        AX_HEADING_ROLE,
        AX_MENU_BAR_ROLE,
        AX_MENU_BAR_ITEM_ROLE,
        AX_MENU_ROLE,
        AX_MENU_ITEM_ROLE,
        AX_SPLIT_GROUP_ROLE,
        AX_SPLITTER_ROLE,
        AX_COLOR_WELL_ROLE,
        AX_TIME_FIELD_ROLE,
        AX_DATE_FIELD_ROLE,
        AX_HELP_TAG_ROLE,
        AX_MATTE_ROLE,
        AX_DOCK_ITEM_ROLE,
        AX_RULER_ROLE,
        AX_RULER_MARKER_ROLE,
        AX_GRID_ROLE,
        AX_LEVEL_INDICATOR_ROLE,
        AX_CELL_ROLE,
        AX_LAYOUT_AREA_ROLE,
        AX_LAYOUT_ITEM_ROLE,
        AX_HANDLE_ROLE,
        AX_POPOVER_ROLE,
    ];
}

/// `ApplicationServices` AX subrole string constants from `AXUIElement.h`.
pub mod subroles {
    /// Mirrors the `ApplicationServices` `kAXCloseButtonSubrole` constant.
    pub const AX_CLOSE_BUTTON_SUBROLE: &str = "AXCloseButton";
    /// Mirrors the `ApplicationServices` `kAXMinimizeButtonSubrole` constant.
    pub const AX_MINIMIZE_BUTTON_SUBROLE: &str = "AXMinimizeButton";
    /// Mirrors the `ApplicationServices` `kAXZoomButtonSubrole` constant.
    pub const AX_ZOOM_BUTTON_SUBROLE: &str = "AXZoomButton";
    /// Mirrors the `ApplicationServices` `kAXToolbarButtonSubrole` constant.
    pub const AX_TOOLBAR_BUTTON_SUBROLE: &str = "AXToolbarButton";
    /// Mirrors the `ApplicationServices` `kAXFullScreenButtonSubrole` constant.
    pub const AX_FULL_SCREEN_BUTTON_SUBROLE: &str = "AXFullScreenButton";
    /// Mirrors the `ApplicationServices` `kAXSecureTextFieldSubrole` constant.
    pub const AX_SECURE_TEXT_FIELD_SUBROLE: &str = "AXSecureTextField";
    /// Mirrors the `ApplicationServices` `kAXTableRowSubrole` constant.
    pub const AX_TABLE_ROW_SUBROLE: &str = "AXTableRow";
    /// Mirrors the `ApplicationServices` `kAXOutlineRowSubrole` constant.
    pub const AX_OUTLINE_ROW_SUBROLE: &str = "AXOutlineRow";
    /// Mirrors the `ApplicationServices` `kAXUnknownSubrole` constant.
    pub const AX_UNKNOWN_SUBROLE: &str = "AXUnknown";
    /// Mirrors the `ApplicationServices` `kAXStandardWindowSubrole` constant.
    pub const AX_STANDARD_WINDOW_SUBROLE: &str = "AXStandardWindow";
    /// Mirrors the `ApplicationServices` `kAXDialogSubrole` constant.
    pub const AX_DIALOG_SUBROLE: &str = "AXDialog";
    /// Mirrors the `ApplicationServices` `kAXSystemDialogSubrole` constant.
    pub const AX_SYSTEM_DIALOG_SUBROLE: &str = "AXSystemDialog";
    /// Mirrors the `ApplicationServices` `kAXFloatingWindowSubrole` constant.
    pub const AX_FLOATING_WINDOW_SUBROLE: &str = "AXFloatingWindow";
    /// Mirrors the `ApplicationServices` `kAXSystemFloatingWindowSubrole` constant.
    pub const AX_SYSTEM_FLOATING_WINDOW_SUBROLE: &str = "AXSystemFloatingWindow";
    /// Mirrors the `ApplicationServices` `kAXDecorativeSubrole` constant.
    pub const AX_DECORATIVE_SUBROLE: &str = "AXDecorative";
    /// Mirrors the `ApplicationServices` `kAXIncrementArrowSubrole` constant.
    pub const AX_INCREMENT_ARROW_SUBROLE: &str = "AXIncrementArrow";
    /// Mirrors the `ApplicationServices` `kAXDecrementArrowSubrole` constant.
    pub const AX_DECREMENT_ARROW_SUBROLE: &str = "AXDecrementArrow";
    /// Mirrors the `ApplicationServices` `kAXIncrementPageSubrole` constant.
    pub const AX_INCREMENT_PAGE_SUBROLE: &str = "AXIncrementPage";
    /// Mirrors the `ApplicationServices` `kAXDecrementPageSubrole` constant.
    pub const AX_DECREMENT_PAGE_SUBROLE: &str = "AXDecrementPage";
    /// Mirrors the `ApplicationServices` `kAXSortButtonSubrole` constant.
    pub const AX_SORT_BUTTON_SUBROLE: &str = "AXSortButton";
    /// Mirrors the `ApplicationServices` `kAXSearchFieldSubrole` constant.
    pub const AX_SEARCH_FIELD_SUBROLE: &str = "AXSearchField";
    /// Mirrors the `ApplicationServices` `kAXTimelineSubrole` constant.
    pub const AX_TIMELINE_SUBROLE: &str = "AXTimeline";
    /// Mirrors the `ApplicationServices` `kAXRatingIndicatorSubrole` constant.
    pub const AX_RATING_INDICATOR_SUBROLE: &str = "AXRatingIndicator";
    /// Mirrors the `ApplicationServices` `kAXContentListSubrole` constant.
    pub const AX_CONTENT_LIST_SUBROLE: &str = "AXContentList";
    /// Mirrors the `ApplicationServices` `kAXDefinitionListSubrole` constant.
    pub const AX_DEFINITION_LIST_SUBROLE: &str = "AXDefinitionList";
    /// Mirrors the `ApplicationServices` `kAXDescriptionListSubrole` constant.
    pub const AX_DESCRIPTION_LIST_SUBROLE: &str = "AXDescriptionList";
    /// Mirrors the `ApplicationServices` `kAXToggleSubrole` constant.
    pub const AX_TOGGLE_SUBROLE: &str = "AXToggle";
    /// Mirrors the `ApplicationServices` `kAXSwitchSubrole` constant.
    pub const AX_SWITCH_SUBROLE: &str = "AXSwitch";
    /// Mirrors the `ApplicationServices` `kAXApplicationDockItemSubrole` constant.
    pub const AX_APPLICATION_DOCK_ITEM_SUBROLE: &str = "AXApplicationDockItem";
    /// Mirrors the `ApplicationServices` `kAXDocumentDockItemSubrole` constant.
    pub const AX_DOCUMENT_DOCK_ITEM_SUBROLE: &str = "AXDocumentDockItem";
    /// Mirrors the `ApplicationServices` `kAXFolderDockItemSubrole` constant.
    pub const AX_FOLDER_DOCK_ITEM_SUBROLE: &str = "AXFolderDockItem";
    /// Mirrors the `ApplicationServices` `kAXMinimizedWindowDockItemSubrole` constant.
    pub const AX_MINIMIZED_WINDOW_DOCK_ITEM_SUBROLE: &str = "AXMinimizedWindowDockItem";
    /// Mirrors the `ApplicationServices` `kAXURLDockItemSubrole` constant.
    pub const AXURL_DOCK_ITEM_SUBROLE: &str = "AXURLDockItem";
    /// Mirrors the `ApplicationServices` `kAXDockExtraDockItemSubrole` constant.
    pub const AX_DOCK_EXTRA_DOCK_ITEM_SUBROLE: &str = "AXDockExtraDockItem";
    /// Mirrors the `ApplicationServices` `kAXTrashDockItemSubrole` constant.
    pub const AX_TRASH_DOCK_ITEM_SUBROLE: &str = "AXTrashDockItem";
    /// Mirrors the `ApplicationServices` `kAXSeparatorDockItemSubrole` constant.
    pub const AX_SEPARATOR_DOCK_ITEM_SUBROLE: &str = "AXSeparatorDockItem";
    /// Mirrors the `ApplicationServices` `kAXProcessSwitcherListSubrole` constant.
    pub const AX_PROCESS_SWITCHER_LIST_SUBROLE: &str = "AXProcessSwitcherList";

    /// Collects the `ApplicationServices` AX subrole constants exposed by this module.
    pub const ALL_SUBROLES: &[&str] = &[
        AX_CLOSE_BUTTON_SUBROLE,
        AX_MINIMIZE_BUTTON_SUBROLE,
        AX_ZOOM_BUTTON_SUBROLE,
        AX_TOOLBAR_BUTTON_SUBROLE,
        AX_FULL_SCREEN_BUTTON_SUBROLE,
        AX_SECURE_TEXT_FIELD_SUBROLE,
        AX_TABLE_ROW_SUBROLE,
        AX_OUTLINE_ROW_SUBROLE,
        AX_UNKNOWN_SUBROLE,
        AX_STANDARD_WINDOW_SUBROLE,
        AX_DIALOG_SUBROLE,
        AX_SYSTEM_DIALOG_SUBROLE,
        AX_FLOATING_WINDOW_SUBROLE,
        AX_SYSTEM_FLOATING_WINDOW_SUBROLE,
        AX_DECORATIVE_SUBROLE,
        AX_INCREMENT_ARROW_SUBROLE,
        AX_DECREMENT_ARROW_SUBROLE,
        AX_INCREMENT_PAGE_SUBROLE,
        AX_DECREMENT_PAGE_SUBROLE,
        AX_SORT_BUTTON_SUBROLE,
        AX_SEARCH_FIELD_SUBROLE,
        AX_TIMELINE_SUBROLE,
        AX_RATING_INDICATOR_SUBROLE,
        AX_CONTENT_LIST_SUBROLE,
        AX_DEFINITION_LIST_SUBROLE,
        AX_DESCRIPTION_LIST_SUBROLE,
        AX_TOGGLE_SUBROLE,
        AX_SWITCH_SUBROLE,
        AX_APPLICATION_DOCK_ITEM_SUBROLE,
        AX_DOCUMENT_DOCK_ITEM_SUBROLE,
        AX_FOLDER_DOCK_ITEM_SUBROLE,
        AX_MINIMIZED_WINDOW_DOCK_ITEM_SUBROLE,
        AXURL_DOCK_ITEM_SUBROLE,
        AX_DOCK_EXTRA_DOCK_ITEM_SUBROLE,
        AX_TRASH_DOCK_ITEM_SUBROLE,
        AX_SEPARATOR_DOCK_ITEM_SUBROLE,
        AX_PROCESS_SWITCHER_LIST_SUBROLE,
    ];
}

/// `ApplicationServices` AX value-string constants from `AXUIElement.h`.
pub mod values {
    /// Mirrors the `ApplicationServices` `kAXHorizontalOrientationValue` constant.
    pub const AX_HORIZONTAL_ORIENTATION_VALUE: &str = "AXHorizontalOrientation";
    /// Mirrors the `ApplicationServices` `kAXVerticalOrientationValue` constant.
    pub const AX_VERTICAL_ORIENTATION_VALUE: &str = "AXVerticalOrientation";
    /// Mirrors the `ApplicationServices` `kAXUnknownOrientationValue` constant.
    pub const AX_UNKNOWN_ORIENTATION_VALUE: &str = "AXUnknownOrientation";
    /// Mirrors the `ApplicationServices` `kAXAscendingSortDirectionValue` constant.
    pub const AX_ASCENDING_SORT_DIRECTION_VALUE: &str = "AXAscendingSortDirection";
    /// Mirrors the `ApplicationServices` `kAXDescendingSortDirectionValue` constant.
    pub const AX_DESCENDING_SORT_DIRECTION_VALUE: &str = "AXDescendingSortDirection";
    /// Mirrors the `ApplicationServices` `kAXUnknownSortDirectionValue` constant.
    pub const AX_UNKNOWN_SORT_DIRECTION_VALUE: &str = "AXUnknownSortDirection";

    /// Collects the `ApplicationServices` AX value constants exposed by this module.
    pub const ALL_VALUE_CONSTANTS: &[&str] = &[
        AX_HORIZONTAL_ORIENTATION_VALUE,
        AX_VERTICAL_ORIENTATION_VALUE,
        AX_UNKNOWN_ORIENTATION_VALUE,
        AX_ASCENDING_SORT_DIRECTION_VALUE,
        AX_DESCENDING_SORT_DIRECTION_VALUE,
        AX_UNKNOWN_SORT_DIRECTION_VALUE,
    ];
}

/// `ApplicationServices` AX menu-item modifier bit constants from `AXUIElement.h`.
pub mod menu_item_modifiers {
    /// Matches `kAXMenuItemModifierNone`.
    pub const NONE: u32 = 0;
    /// Matches `kAXMenuItemModifierShift`.
    pub const SHIFT: u32 = 1;
    /// Matches `kAXMenuItemModifierOption`.
    pub const OPTION: u32 = 2;
    /// Matches `kAXMenuItemModifierControl`.
    pub const CONTROL: u32 = 4;
    /// Matches `kAXMenuItemModifierNoCommand`.
    pub const NO_COMMAND: u32 = 8;
}

/// Re-exports the `ApplicationServices` AX attribute constants.
pub use attributes::*;
/// Re-exports the `ApplicationServices` AX menu-item modifier constants.
pub use menu_item_modifiers::*;
/// Re-exports the `ApplicationServices` AX parameterized-attribute constants.
pub use parameterized::*;
/// Re-exports the `ApplicationServices` AX role constants.
pub use roles::*;
/// Re-exports the `ApplicationServices` AX subrole constants.
pub use subroles::*;
/// Re-exports the `ApplicationServices` AX value constants.
pub use values::*;
