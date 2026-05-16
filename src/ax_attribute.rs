//! Accessibility attribute, role, subrole, and value constants.

pub mod attributes {
    pub const AX_ROLE_ATTRIBUTE: &str = "AXRole";
    pub const AX_SUBROLE_ATTRIBUTE: &str = "AXSubrole";
    pub const AX_ROLE_DESCRIPTION_ATTRIBUTE: &str = "AXRoleDescription";
    pub const AX_HELP_ATTRIBUTE: &str = "AXHelp";
    pub const AX_TITLE_ATTRIBUTE: &str = "AXTitle";
    pub const AX_VALUE_ATTRIBUTE: &str = "AXValue";
    pub const AX_VALUE_DESCRIPTION_ATTRIBUTE: &str = "AXValueDescription";
    pub const AX_MIN_VALUE_ATTRIBUTE: &str = "AXMinValue";
    pub const AX_MAX_VALUE_ATTRIBUTE: &str = "AXMaxValue";
    pub const AX_VALUE_INCREMENT_ATTRIBUTE: &str = "AXValueIncrement";
    pub const AX_ALLOWED_VALUES_ATTRIBUTE: &str = "AXAllowedValues";
    pub const AX_PLACEHOLDER_VALUE_ATTRIBUTE: &str = "AXPlaceholderValue";
    pub const AX_ENABLED_ATTRIBUTE: &str = "AXEnabled";
    pub const AX_ELEMENT_BUSY_ATTRIBUTE: &str = "AXElementBusy";
    pub const AX_FOCUSED_ATTRIBUTE: &str = "AXFocused";
    pub const AX_PARENT_ATTRIBUTE: &str = "AXParent";
    pub const AX_CHILDREN_ATTRIBUTE: &str = "AXChildren";
    pub const AX_SELECTED_CHILDREN_ATTRIBUTE: &str = "AXSelectedChildren";
    pub const AX_VISIBLE_CHILDREN_ATTRIBUTE: &str = "AXVisibleChildren";
    pub const AX_WINDOW_ATTRIBUTE: &str = "AXWindow";
    pub const AX_TOP_LEVEL_UI_ELEMENT_ATTRIBUTE: &str = "AXTopLevelUIElement";
    pub const AX_POSITION_ATTRIBUTE: &str = "AXPosition";
    pub const AX_SIZE_ATTRIBUTE: &str = "AXSize";
    pub const AX_ORIENTATION_ATTRIBUTE: &str = "AXOrientation";
    pub const AX_DESCRIPTION_ATTRIBUTE: &str = "AXDescription";
    pub const AX_DESCRIPTION: &str = "AXDescription";
    pub const AX_SELECTED_TEXT_ATTRIBUTE: &str = "AXSelectedText";
    pub const AX_SELECTED_TEXT_RANGE_ATTRIBUTE: &str = "AXSelectedTextRange";
    pub const AX_SELECTED_TEXT_RANGES_ATTRIBUTE: &str = "AXSelectedTextRanges";
    pub const AX_VISIBLE_CHARACTER_RANGE_ATTRIBUTE: &str = "AXVisibleCharacterRange";
    pub const AX_NUMBER_OF_CHARACTERS_ATTRIBUTE: &str = "AXNumberOfCharacters";
    pub const AX_SHARED_TEXT_UI_ELEMENTS_ATTRIBUTE: &str = "AXSharedTextUIElements";
    pub const AX_SHARED_CHARACTER_RANGE_ATTRIBUTE: &str = "AXSharedCharacterRange";
    pub const AX_SHARED_FOCUS_ELEMENTS_ATTRIBUTE: &str = "AXSharedFocusElements";
    pub const AX_INSERTION_POINT_LINE_NUMBER_ATTRIBUTE: &str = "AXInsertionPointLineNumber";
    pub const AX_MAIN_ATTRIBUTE: &str = "AXMain";
    pub const AX_MINIMIZED_ATTRIBUTE: &str = "AXMinimized";
    pub const AX_CLOSE_BUTTON_ATTRIBUTE: &str = "AXCloseButton";
    pub const AX_ZOOM_BUTTON_ATTRIBUTE: &str = "AXZoomButton";
    pub const AX_MINIMIZE_BUTTON_ATTRIBUTE: &str = "AXMinimizeButton";
    pub const AX_TOOLBAR_BUTTON_ATTRIBUTE: &str = "AXToolbarButton";
    pub const AX_FULL_SCREEN_BUTTON_ATTRIBUTE: &str = "AXFullScreenButton";
    pub const AX_PROXY_ATTRIBUTE: &str = "AXProxy";
    pub const AX_GROW_AREA_ATTRIBUTE: &str = "AXGrowArea";
    pub const AX_MODAL_ATTRIBUTE: &str = "AXModal";
    pub const AX_DEFAULT_BUTTON_ATTRIBUTE: &str = "AXDefaultButton";
    pub const AX_CANCEL_BUTTON_ATTRIBUTE: &str = "AXCancelButton";
    pub const AX_MENU_ITEM_CMD_CHAR_ATTRIBUTE: &str = "AXMenuItemCmdChar";
    pub const AX_MENU_ITEM_CMD_VIRTUAL_KEY_ATTRIBUTE: &str = "AXMenuItemCmdVirtualKey";
    pub const AX_MENU_ITEM_CMD_GLYPH_ATTRIBUTE: &str = "AXMenuItemCmdGlyph";
    pub const AX_MENU_ITEM_CMD_MODIFIERS_ATTRIBUTE: &str = "AXMenuItemCmdModifiers";
    pub const AX_MENU_ITEM_MARK_CHAR_ATTRIBUTE: &str = "AXMenuItemMarkChar";
    pub const AX_MENU_ITEM_PRIMARY_UI_ELEMENT_ATTRIBUTE: &str = "AXMenuItemPrimaryUIElement";
    pub const AX_MENU_BAR_ATTRIBUTE: &str = "AXMenuBar";
    pub const AX_WINDOWS_ATTRIBUTE: &str = "AXWindows";
    pub const AX_FRONTMOST_ATTRIBUTE: &str = "AXFrontmost";
    pub const AX_HIDDEN_ATTRIBUTE: &str = "AXHidden";
    pub const AX_MAIN_WINDOW_ATTRIBUTE: &str = "AXMainWindow";
    pub const AX_FOCUSED_WINDOW_ATTRIBUTE: &str = "AXFocusedWindow";
    pub const AX_FOCUSED_UI_ELEMENT_ATTRIBUTE: &str = "AXFocusedUIElement";
    pub const AX_EXTRAS_MENU_BAR_ATTRIBUTE: &str = "AXExtrasMenuBar";
    pub const AX_HEADER_ATTRIBUTE: &str = "AXHeader";
    pub const AX_EDITED_ATTRIBUTE: &str = "AXEdited";
    pub const AX_VALUE_WRAPS_ATTRIBUTE: &str = "AXValueWraps";
    pub const AX_TABS_ATTRIBUTE: &str = "AXTabs";
    pub const AX_TITLE_UI_ELEMENT_ATTRIBUTE: &str = "AXTitleUIElement";
    pub const AX_HORIZONTAL_SCROLL_BAR_ATTRIBUTE: &str = "AXHorizontalScrollBar";
    pub const AX_VERTICAL_SCROLL_BAR_ATTRIBUTE: &str = "AXVerticalScrollBar";
    pub const AX_OVERFLOW_BUTTON_ATTRIBUTE: &str = "AXOverflowButton";
    pub const AX_FILENAME_ATTRIBUTE: &str = "AXFilename";
    pub const AX_EXPANDED_ATTRIBUTE: &str = "AXExpanded";
    pub const AX_SELECTED_ATTRIBUTE: &str = "AXSelected";
    pub const AX_SPLITTERS_ATTRIBUTE: &str = "AXSplitters";
    pub const AX_NEXT_CONTENTS_ATTRIBUTE: &str = "AXNextContents";
    pub const AX_DOCUMENT_ATTRIBUTE: &str = "AXDocument";
    pub const AX_DECREMENT_BUTTON_ATTRIBUTE: &str = "AXDecrementButton";
    pub const AX_INCREMENT_BUTTON_ATTRIBUTE: &str = "AXIncrementButton";
    pub const AX_PREVIOUS_CONTENTS_ATTRIBUTE: &str = "AXPreviousContents";
    pub const AX_CONTENTS_ATTRIBUTE: &str = "AXContents";
    pub const AX_INCREMENTOR_ATTRIBUTE: &str = "AXIncrementor";
    pub const AX_HOUR_FIELD_ATTRIBUTE: &str = "AXHourField";
    pub const AX_MINUTE_FIELD_ATTRIBUTE: &str = "AXMinuteField";
    pub const AX_SECOND_FIELD_ATTRIBUTE: &str = "AXSecondField";
    pub const AXAMPM_FIELD_ATTRIBUTE: &str = "AXAMPMField";
    pub const AX_DAY_FIELD_ATTRIBUTE: &str = "AXDayField";
    pub const AX_MONTH_FIELD_ATTRIBUTE: &str = "AXMonthField";
    pub const AX_YEAR_FIELD_ATTRIBUTE: &str = "AXYearField";
    pub const AX_COLUMN_TITLE_ATTRIBUTE: &str = "AXColumnTitles";
    pub const AXURL_ATTRIBUTE: &str = "AXURL";
    pub const AX_LABEL_UI_ELEMENTS_ATTRIBUTE: &str = "AXLabelUIElements";
    pub const AX_LABEL_VALUE_ATTRIBUTE: &str = "AXLabelValue";
    pub const AX_SHOWN_MENU_UI_ELEMENT_ATTRIBUTE: &str = "AXShownMenuUIElement";
    pub const AX_SERVES_AS_TITLE_FOR_UI_ELEMENTS_ATTRIBUTE: &str = "AXServesAsTitleForUIElements";
    pub const AX_LINKED_UI_ELEMENTS_ATTRIBUTE: &str = "AXLinkedUIElements";
    pub const AX_ROWS_ATTRIBUTE: &str = "AXRows";
    pub const AX_VISIBLE_ROWS_ATTRIBUTE: &str = "AXVisibleRows";
    pub const AX_SELECTED_ROWS_ATTRIBUTE: &str = "AXSelectedRows";
    pub const AX_COLUMNS_ATTRIBUTE: &str = "AXColumns";
    pub const AX_VISIBLE_COLUMNS_ATTRIBUTE: &str = "AXVisibleColumns";
    pub const AX_SELECTED_COLUMNS_ATTRIBUTE: &str = "AXSelectedColumns";
    pub const AX_SORT_DIRECTION_ATTRIBUTE: &str = "AXSortDirection";
    pub const AX_INDEX_ATTRIBUTE: &str = "AXIndex";
    pub const AX_DISCLOSING_ATTRIBUTE: &str = "AXDisclosing";
    pub const AX_DISCLOSED_ROWS_ATTRIBUTE: &str = "AXDisclosedRows";
    pub const AX_DISCLOSED_BY_ROW_ATTRIBUTE: &str = "AXDisclosedByRow";
    pub const AX_DISCLOSURE_LEVEL_ATTRIBUTE: &str = "AXDisclosureLevel";
    pub const AX_MATTE_HOLE_ATTRIBUTE: &str = "AXMatteHole";
    pub const AX_MATTE_CONTENT_UI_ELEMENT_ATTRIBUTE: &str = "AXMatteContentUIElement";
    pub const AX_MARKER_UI_ELEMENTS_ATTRIBUTE: &str = "AXMarkerUIElements";
    pub const AX_UNITS_ATTRIBUTE: &str = "AXUnits";
    pub const AX_UNIT_DESCRIPTION_ATTRIBUTE: &str = "AXUnitDescription";
    pub const AX_MARKER_TYPE_ATTRIBUTE: &str = "AXMarkerType";
    pub const AX_MARKER_TYPE_DESCRIPTION_ATTRIBUTE: &str = "AXMarkerTypeDescription";
    pub const AX_IS_APPLICATION_RUNNING_ATTRIBUTE: &str = "AXIsApplicationRunning";
    pub const AX_SEARCH_BUTTON_ATTRIBUTE: &str = "AXSearchButton";
    pub const AX_CLEAR_BUTTON_ATTRIBUTE: &str = "AXClearButton";
    pub const AX_FOCUSED_APPLICATION_ATTRIBUTE: &str = "AXFocusedApplication";
    pub const AX_ROW_COUNT_ATTRIBUTE: &str = "AXRowCount";
    pub const AX_COLUMN_COUNT_ATTRIBUTE: &str = "AXColumnCount";
    pub const AX_ORDERED_BY_ROW_ATTRIBUTE: &str = "AXOrderedByRow";
    pub const AX_WARNING_VALUE_ATTRIBUTE: &str = "AXWarningValue";
    pub const AX_CRITICAL_VALUE_ATTRIBUTE: &str = "AXCriticalValue";
    pub const AX_SELECTED_CELLS_ATTRIBUTE: &str = "AXSelectedCells";
    pub const AX_VISIBLE_CELLS_ATTRIBUTE: &str = "AXVisibleCells";
    pub const AX_ROW_HEADER_UI_ELEMENTS_ATTRIBUTE: &str = "AXRowHeaderUIElements";
    pub const AX_COLUMN_HEADER_UI_ELEMENTS_ATTRIBUTE: &str = "AXColumnHeaderUIElements";
    pub const AX_ROW_INDEX_RANGE_ATTRIBUTE: &str = "AXRowIndexRange";
    pub const AX_COLUMN_INDEX_RANGE_ATTRIBUTE: &str = "AXColumnIndexRange";
    pub const AX_HORIZONTAL_UNITS_ATTRIBUTE: &str = "AXHorizontalUnits";
    pub const AX_VERTICAL_UNITS_ATTRIBUTE: &str = "AXVerticalUnits";
    pub const AX_HORIZONTAL_UNIT_DESCRIPTION_ATTRIBUTE: &str = "AXHorizontalUnitDescription";
    pub const AX_VERTICAL_UNIT_DESCRIPTION_ATTRIBUTE: &str = "AXVerticalUnitDescription";
    pub const AX_HANDLES_ATTRIBUTE: &str = "AXHandles";
    pub const AX_TEXT_ATTRIBUTE: &str = "AXText";
    pub const AX_VISIBLE_TEXT_ATTRIBUTE: &str = "AXVisibleText";
    pub const AX_IS_EDITABLE_ATTRIBUTE: &str = "AXIsEditable";
    pub const AX_COLUMN_TITLES_ATTRIBUTE: &str = "AXColumnTitles";
    pub const AX_IDENTIFIER_ATTRIBUTE: &str = "AXIdentifier";
    pub const AX_ALTERNATE_UI_VISIBLE_ATTRIBUTE: &str = "AXAlternateUIVisible";

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

pub mod parameterized {
    pub const AX_LINE_FOR_INDEX_PARAMETERIZED_ATTRIBUTE: &str = "AXLineForIndex";
    pub const AX_RANGE_FOR_LINE_PARAMETERIZED_ATTRIBUTE: &str = "AXRangeForLine";
    pub const AX_STRING_FOR_RANGE_PARAMETERIZED_ATTRIBUTE: &str = "AXStringForRange";
    pub const AX_RANGE_FOR_POSITION_PARAMETERIZED_ATTRIBUTE: &str = "AXRangeForPosition";
    pub const AX_RANGE_FOR_INDEX_PARAMETERIZED_ATTRIBUTE: &str = "AXRangeForIndex";
    pub const AX_BOUNDS_FOR_RANGE_PARAMETERIZED_ATTRIBUTE: &str = "AXBoundsForRange";
    pub const AXRTF_FOR_RANGE_PARAMETERIZED_ATTRIBUTE: &str = "AXRTFForRange";
    pub const AX_ATTRIBUTED_STRING_FOR_RANGE_PARAMETERIZED_ATTRIBUTE: &str =
        "AXAttributedStringForRange";
    pub const AX_STYLE_RANGE_FOR_INDEX_PARAMETERIZED_ATTRIBUTE: &str = "AXStyleRangeForIndex";
    pub const AX_CELL_FOR_COLUMN_AND_ROW_PARAMETERIZED_ATTRIBUTE: &str = "AXCellForColumnAndRow";
    pub const AX_LAYOUT_POINT_FOR_SCREEN_POINT_PARAMETERIZED_ATTRIBUTE: &str =
        "AXLayoutPointForScreenPoint";
    pub const AX_LAYOUT_SIZE_FOR_SCREEN_SIZE_PARAMETERIZED_ATTRIBUTE: &str =
        "AXLayoutSizeForScreenSize";
    pub const AX_SCREEN_POINT_FOR_LAYOUT_POINT_PARAMETERIZED_ATTRIBUTE: &str =
        "AXScreenPointForLayoutPoint";
    pub const AX_SCREEN_SIZE_FOR_LAYOUT_SIZE_PARAMETERIZED_ATTRIBUTE: &str =
        "AXScreenSizeForLayoutSize";

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

pub mod roles {
    pub const AX_APPLICATION_ROLE: &str = "AXApplication";
    pub const AX_SYSTEM_WIDE_ROLE: &str = "AXSystemWide";
    pub const AX_WINDOW_ROLE: &str = "AXWindow";
    pub const AX_SHEET_ROLE: &str = "AXSheet";
    pub const AX_DRAWER_ROLE: &str = "AXDrawer";
    pub const AX_GROW_AREA_ROLE: &str = "AXGrowArea";
    pub const AX_IMAGE_ROLE: &str = "AXImage";
    pub const AX_UNKNOWN_ROLE: &str = "AXUnknown";
    pub const AX_BUTTON_ROLE: &str = "AXButton";
    pub const AX_RADIO_BUTTON_ROLE: &str = "AXRadioButton";
    pub const AX_CHECK_BOX_ROLE: &str = "AXCheckBox";
    pub const AX_POP_UP_BUTTON_ROLE: &str = "AXPopUpButton";
    pub const AX_MENU_BUTTON_ROLE: &str = "AXMenuButton";
    pub const AX_TAB_GROUP_ROLE: &str = "AXTabGroup";
    pub const AX_TABLE_ROLE: &str = "AXTable";
    pub const AX_COLUMN_ROLE: &str = "AXColumn";
    pub const AX_ROW_ROLE: &str = "AXRow";
    pub const AX_OUTLINE_ROLE: &str = "AXOutline";
    pub const AX_BROWSER_ROLE: &str = "AXBrowser";
    pub const AX_SCROLL_AREA_ROLE: &str = "AXScrollArea";
    pub const AX_SCROLL_BAR_ROLE: &str = "AXScrollBar";
    pub const AX_RADIO_GROUP_ROLE: &str = "AXRadioGroup";
    pub const AX_LIST_ROLE: &str = "AXList";
    pub const AX_GROUP_ROLE: &str = "AXGroup";
    pub const AX_VALUE_INDICATOR_ROLE: &str = "AXValueIndicator";
    pub const AX_COMBO_BOX_ROLE: &str = "AXComboBox";
    pub const AX_SLIDER_ROLE: &str = "AXSlider";
    pub const AX_INCREMENTOR_ROLE: &str = "AXIncrementor";
    pub const AX_BUSY_INDICATOR_ROLE: &str = "AXBusyIndicator";
    pub const AX_PROGRESS_INDICATOR_ROLE: &str = "AXProgressIndicator";
    pub const AX_RELEVANCE_INDICATOR_ROLE: &str = "AXRelevanceIndicator";
    pub const AX_TOOLBAR_ROLE: &str = "AXToolbar";
    pub const AX_DISCLOSURE_TRIANGLE_ROLE: &str = "AXDisclosureTriangle";
    pub const AX_TEXT_FIELD_ROLE: &str = "AXTextField";
    pub const AX_TEXT_AREA_ROLE: &str = "AXTextArea";
    pub const AX_STATIC_TEXT_ROLE: &str = "AXStaticText";
    pub const AX_HEADING_ROLE: &str = "AXHeading";
    pub const AX_MENU_BAR_ROLE: &str = "AXMenuBar";
    pub const AX_MENU_BAR_ITEM_ROLE: &str = "AXMenuBarItem";
    pub const AX_MENU_ROLE: &str = "AXMenu";
    pub const AX_MENU_ITEM_ROLE: &str = "AXMenuItem";
    pub const AX_SPLIT_GROUP_ROLE: &str = "AXSplitGroup";
    pub const AX_SPLITTER_ROLE: &str = "AXSplitter";
    pub const AX_COLOR_WELL_ROLE: &str = "AXColorWell";
    pub const AX_TIME_FIELD_ROLE: &str = "AXTimeField";
    pub const AX_DATE_FIELD_ROLE: &str = "AXDateField";
    pub const AX_HELP_TAG_ROLE: &str = "AXHelpTag";
    pub const AX_MATTE_ROLE: &str = "AXMatte";
    pub const AX_DOCK_ITEM_ROLE: &str = "AXDockItem";
    pub const AX_RULER_ROLE: &str = "AXRuler";
    pub const AX_RULER_MARKER_ROLE: &str = "AXRulerMarker";
    pub const AX_GRID_ROLE: &str = "AXGrid";
    pub const AX_LEVEL_INDICATOR_ROLE: &str = "AXLevelIndicator";
    pub const AX_CELL_ROLE: &str = "AXCell";
    pub const AX_LAYOUT_AREA_ROLE: &str = "AXLayoutArea";
    pub const AX_LAYOUT_ITEM_ROLE: &str = "AXLayoutItem";
    pub const AX_HANDLE_ROLE: &str = "AXHandle";
    pub const AX_POPOVER_ROLE: &str = "AXPopover";

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

pub mod subroles {
    pub const AX_CLOSE_BUTTON_SUBROLE: &str = "AXCloseButton";
    pub const AX_MINIMIZE_BUTTON_SUBROLE: &str = "AXMinimizeButton";
    pub const AX_ZOOM_BUTTON_SUBROLE: &str = "AXZoomButton";
    pub const AX_TOOLBAR_BUTTON_SUBROLE: &str = "AXToolbarButton";
    pub const AX_FULL_SCREEN_BUTTON_SUBROLE: &str = "AXFullScreenButton";
    pub const AX_SECURE_TEXT_FIELD_SUBROLE: &str = "AXSecureTextField";
    pub const AX_TABLE_ROW_SUBROLE: &str = "AXTableRow";
    pub const AX_OUTLINE_ROW_SUBROLE: &str = "AXOutlineRow";
    pub const AX_UNKNOWN_SUBROLE: &str = "AXUnknown";
    pub const AX_STANDARD_WINDOW_SUBROLE: &str = "AXStandardWindow";
    pub const AX_DIALOG_SUBROLE: &str = "AXDialog";
    pub const AX_SYSTEM_DIALOG_SUBROLE: &str = "AXSystemDialog";
    pub const AX_FLOATING_WINDOW_SUBROLE: &str = "AXFloatingWindow";
    pub const AX_SYSTEM_FLOATING_WINDOW_SUBROLE: &str = "AXSystemFloatingWindow";
    pub const AX_DECORATIVE_SUBROLE: &str = "AXDecorative";
    pub const AX_INCREMENT_ARROW_SUBROLE: &str = "AXIncrementArrow";
    pub const AX_DECREMENT_ARROW_SUBROLE: &str = "AXDecrementArrow";
    pub const AX_INCREMENT_PAGE_SUBROLE: &str = "AXIncrementPage";
    pub const AX_DECREMENT_PAGE_SUBROLE: &str = "AXDecrementPage";
    pub const AX_SORT_BUTTON_SUBROLE: &str = "AXSortButton";
    pub const AX_SEARCH_FIELD_SUBROLE: &str = "AXSearchField";
    pub const AX_TIMELINE_SUBROLE: &str = "AXTimeline";
    pub const AX_RATING_INDICATOR_SUBROLE: &str = "AXRatingIndicator";
    pub const AX_CONTENT_LIST_SUBROLE: &str = "AXContentList";
    pub const AX_DEFINITION_LIST_SUBROLE: &str = "AXDefinitionList";
    pub const AX_DESCRIPTION_LIST_SUBROLE: &str = "AXDescriptionList";
    pub const AX_TOGGLE_SUBROLE: &str = "AXToggle";
    pub const AX_SWITCH_SUBROLE: &str = "AXSwitch";
    pub const AX_APPLICATION_DOCK_ITEM_SUBROLE: &str = "AXApplicationDockItem";
    pub const AX_DOCUMENT_DOCK_ITEM_SUBROLE: &str = "AXDocumentDockItem";
    pub const AX_FOLDER_DOCK_ITEM_SUBROLE: &str = "AXFolderDockItem";
    pub const AX_MINIMIZED_WINDOW_DOCK_ITEM_SUBROLE: &str = "AXMinimizedWindowDockItem";
    pub const AXURL_DOCK_ITEM_SUBROLE: &str = "AXURLDockItem";
    pub const AX_DOCK_EXTRA_DOCK_ITEM_SUBROLE: &str = "AXDockExtraDockItem";
    pub const AX_TRASH_DOCK_ITEM_SUBROLE: &str = "AXTrashDockItem";
    pub const AX_SEPARATOR_DOCK_ITEM_SUBROLE: &str = "AXSeparatorDockItem";
    pub const AX_PROCESS_SWITCHER_LIST_SUBROLE: &str = "AXProcessSwitcherList";

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

pub mod values {
    pub const AX_HORIZONTAL_ORIENTATION_VALUE: &str = "AXHorizontalOrientation";
    pub const AX_VERTICAL_ORIENTATION_VALUE: &str = "AXVerticalOrientation";
    pub const AX_UNKNOWN_ORIENTATION_VALUE: &str = "AXUnknownOrientation";
    pub const AX_ASCENDING_SORT_DIRECTION_VALUE: &str = "AXAscendingSortDirection";
    pub const AX_DESCENDING_SORT_DIRECTION_VALUE: &str = "AXDescendingSortDirection";
    pub const AX_UNKNOWN_SORT_DIRECTION_VALUE: &str = "AXUnknownSortDirection";

    pub const ALL_VALUE_CONSTANTS: &[&str] = &[
        AX_HORIZONTAL_ORIENTATION_VALUE,
        AX_VERTICAL_ORIENTATION_VALUE,
        AX_UNKNOWN_ORIENTATION_VALUE,
        AX_ASCENDING_SORT_DIRECTION_VALUE,
        AX_DESCENDING_SORT_DIRECTION_VALUE,
        AX_UNKNOWN_SORT_DIRECTION_VALUE,
    ];
}

pub mod menu_item_modifiers {
    pub const NONE: u32 = 0;
    pub const SHIFT: u32 = 1;
    pub const OPTION: u32 = 2;
    pub const CONTROL: u32 = 4;
    pub const NO_COMMAND: u32 = 8;
}

pub use attributes::*;
pub use menu_item_modifiers::*;
pub use parameterized::*;
pub use roles::*;
pub use subroles::*;
pub use values::*;
