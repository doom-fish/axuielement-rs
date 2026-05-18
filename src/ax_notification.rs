//! Accessibility notification constants and priorities.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
/// Announcement priority values used by the `ApplicationServices` notification-info keys.
pub enum AXPriority {
    /// Matches the low announcement priority used with `kAXPriorityKey`.
    Low = 10,
    /// Matches the medium announcement priority used with `kAXPriorityKey`.
    Medium = 50,
    /// Matches the high announcement priority used with `kAXPriorityKey`.
    High = 90,
}

impl AXPriority {
    #[must_use]
    /// Returns the raw priority value stored alongside `kAXPriorityKey`.
    pub const fn raw(self) -> isize {
        self as isize
    }
}

/// Mirrors the `ApplicationServices` `kAXMainWindowChangedNotification` constant.
pub const AX_MAIN_WINDOW_CHANGED_NOTIFICATION: &str = "AXMainWindowChanged";
/// Mirrors the `ApplicationServices` `kAXFocusedWindowChangedNotification` constant.
pub const AX_FOCUSED_WINDOW_CHANGED_NOTIFICATION: &str = "AXFocusedWindowChanged";
/// Mirrors the `ApplicationServices` `kAXFocusedUIElementChangedNotification` constant.
pub const AX_FOCUSED_UI_ELEMENT_CHANGED_NOTIFICATION: &str = "AXFocusedUIElementChanged";
/// Mirrors the `ApplicationServices` `kAXApplicationActivatedNotification` constant.
pub const AX_APPLICATION_ACTIVATED_NOTIFICATION: &str = "AXApplicationActivated";
/// Mirrors the `ApplicationServices` `kAXApplicationDeactivatedNotification` constant.
pub const AX_APPLICATION_DEACTIVATED_NOTIFICATION: &str = "AXApplicationDeactivated";
/// Mirrors the `ApplicationServices` `kAXApplicationHiddenNotification` constant.
pub const AX_APPLICATION_HIDDEN_NOTIFICATION: &str = "AXApplicationHidden";
/// Mirrors the `ApplicationServices` `kAXApplicationShownNotification` constant.
pub const AX_APPLICATION_SHOWN_NOTIFICATION: &str = "AXApplicationShown";
/// Mirrors the `ApplicationServices` `kAXWindowCreatedNotification` constant.
pub const AX_WINDOW_CREATED_NOTIFICATION: &str = "AXWindowCreated";
/// Mirrors the `ApplicationServices` `kAXWindowMovedNotification` constant.
pub const AX_WINDOW_MOVED_NOTIFICATION: &str = "AXWindowMoved";
/// Mirrors the `ApplicationServices` `kAXWindowResizedNotification` constant.
pub const AX_WINDOW_RESIZED_NOTIFICATION: &str = "AXWindowResized";
/// Mirrors the `ApplicationServices` `kAXWindowMiniaturizedNotification` constant.
pub const AX_WINDOW_MINIATURIZED_NOTIFICATION: &str = "AXWindowMiniaturized";
/// Mirrors the `ApplicationServices` `kAXWindowDeminiaturizedNotification` constant.
pub const AX_WINDOW_DEMINIATURIZED_NOTIFICATION: &str = "AXWindowDeminiaturized";
/// Mirrors the `ApplicationServices` `kAXDrawerCreatedNotification` constant.
pub const AX_DRAWER_CREATED_NOTIFICATION: &str = "AXDrawerCreated";
/// Mirrors the `ApplicationServices` `kAXSheetCreatedNotification` constant.
pub const AX_SHEET_CREATED_NOTIFICATION: &str = "AXSheetCreated";
/// Mirrors the `ApplicationServices` `kAXHelpTagCreatedNotification` constant.
pub const AX_HELP_TAG_CREATED_NOTIFICATION: &str = "AXHelpTagCreated";
/// Mirrors the `ApplicationServices` `kAXValueChangedNotification` constant.
pub const AX_VALUE_CHANGED_NOTIFICATION: &str = "AXValueChanged";
/// Mirrors the `ApplicationServices` `kAXUIElementDestroyedNotification` constant.
pub const AXUI_ELEMENT_DESTROYED_NOTIFICATION: &str = "AXUIElementDestroyed";
/// Mirrors the `ApplicationServices` `kAXElementBusyChangedNotification` constant.
pub const AX_ELEMENT_BUSY_CHANGED_NOTIFICATION: &str = "AXElementBusyChanged";
/// Mirrors the `ApplicationServices` `kAXMenuOpenedNotification` constant.
pub const AX_MENU_OPENED_NOTIFICATION: &str = "AXMenuOpened";
/// Mirrors the `ApplicationServices` `kAXMenuClosedNotification` constant.
pub const AX_MENU_CLOSED_NOTIFICATION: &str = "AXMenuClosed";
/// Mirrors the `ApplicationServices` `kAXMenuItemSelectedNotification` constant.
pub const AX_MENU_ITEM_SELECTED_NOTIFICATION: &str = "AXMenuItemSelected";
/// Mirrors the `ApplicationServices` `kAXRowCountChangedNotification` constant.
pub const AX_ROW_COUNT_CHANGED_NOTIFICATION: &str = "AXRowCountChanged";
/// Mirrors the `ApplicationServices` `kAXRowExpandedNotification` constant.
pub const AX_ROW_EXPANDED_NOTIFICATION: &str = "AXRowExpanded";
/// Mirrors the `ApplicationServices` `kAXRowCollapsedNotification` constant.
pub const AX_ROW_COLLAPSED_NOTIFICATION: &str = "AXRowCollapsed";
/// Mirrors the `ApplicationServices` `kAXSelectedCellsChangedNotification` constant.
pub const AX_SELECTED_CELLS_CHANGED_NOTIFICATION: &str = "AXSelectedCellsChanged";
/// Mirrors the `ApplicationServices` `kAXUnitsChangedNotification` constant.
pub const AX_UNITS_CHANGED_NOTIFICATION: &str = "AXUnitsChanged";
/// Mirrors the `ApplicationServices` `kAXSelectedChildrenMovedNotification` constant.
pub const AX_SELECTED_CHILDREN_MOVED_NOTIFICATION: &str = "AXSelectedChildrenMoved";
/// Mirrors the `ApplicationServices` `kAXSelectedChildrenChangedNotification` constant.
pub const AX_SELECTED_CHILDREN_CHANGED_NOTIFICATION: &str = "AXSelectedChildrenChanged";
/// Mirrors the `ApplicationServices` `kAXResizedNotification` constant.
pub const AX_RESIZED_NOTIFICATION: &str = "AXResized";
/// Mirrors the `ApplicationServices` `kAXMovedNotification` constant.
pub const AX_MOVED_NOTIFICATION: &str = "AXMoved";
/// Mirrors the `ApplicationServices` `kAXCreatedNotification` constant.
pub const AX_CREATED_NOTIFICATION: &str = "AXCreated";
/// Mirrors the `ApplicationServices` `kAXSelectedRowsChangedNotification` constant.
pub const AX_SELECTED_ROWS_CHANGED_NOTIFICATION: &str = "AXSelectedRowsChanged";
/// Mirrors the `ApplicationServices` `kAXSelectedColumnsChangedNotification` constant.
pub const AX_SELECTED_COLUMNS_CHANGED_NOTIFICATION: &str = "AXSelectedColumnsChanged";
/// Mirrors the `ApplicationServices` `kAXSelectedTextChangedNotification` constant.
pub const AX_SELECTED_TEXT_CHANGED_NOTIFICATION: &str = "AXSelectedTextChanged";
/// Mirrors the `ApplicationServices` `kAXTitleChangedNotification` constant.
pub const AX_TITLE_CHANGED_NOTIFICATION: &str = "AXTitleChanged";
/// Mirrors the `ApplicationServices` `kAXLayoutChangedNotification` constant.
pub const AX_LAYOUT_CHANGED_NOTIFICATION: &str = "AXLayoutChanged";
/// Mirrors the `ApplicationServices` `kAXAnnouncementRequestedNotification` constant.
pub const AX_ANNOUNCEMENT_REQUESTED_NOTIFICATION: &str = "AXAnnouncementRequested";

/// Collects the `ApplicationServices` AX notification constants exposed by this module.
pub const ALL_NOTIFICATIONS: &[&str] = &[
    AX_MAIN_WINDOW_CHANGED_NOTIFICATION,
    AX_FOCUSED_WINDOW_CHANGED_NOTIFICATION,
    AX_FOCUSED_UI_ELEMENT_CHANGED_NOTIFICATION,
    AX_APPLICATION_ACTIVATED_NOTIFICATION,
    AX_APPLICATION_DEACTIVATED_NOTIFICATION,
    AX_APPLICATION_HIDDEN_NOTIFICATION,
    AX_APPLICATION_SHOWN_NOTIFICATION,
    AX_WINDOW_CREATED_NOTIFICATION,
    AX_WINDOW_MOVED_NOTIFICATION,
    AX_WINDOW_RESIZED_NOTIFICATION,
    AX_WINDOW_MINIATURIZED_NOTIFICATION,
    AX_WINDOW_DEMINIATURIZED_NOTIFICATION,
    AX_DRAWER_CREATED_NOTIFICATION,
    AX_SHEET_CREATED_NOTIFICATION,
    AX_HELP_TAG_CREATED_NOTIFICATION,
    AX_VALUE_CHANGED_NOTIFICATION,
    AXUI_ELEMENT_DESTROYED_NOTIFICATION,
    AX_ELEMENT_BUSY_CHANGED_NOTIFICATION,
    AX_MENU_OPENED_NOTIFICATION,
    AX_MENU_CLOSED_NOTIFICATION,
    AX_MENU_ITEM_SELECTED_NOTIFICATION,
    AX_ROW_COUNT_CHANGED_NOTIFICATION,
    AX_ROW_EXPANDED_NOTIFICATION,
    AX_ROW_COLLAPSED_NOTIFICATION,
    AX_SELECTED_CELLS_CHANGED_NOTIFICATION,
    AX_UNITS_CHANGED_NOTIFICATION,
    AX_SELECTED_CHILDREN_MOVED_NOTIFICATION,
    AX_SELECTED_CHILDREN_CHANGED_NOTIFICATION,
    AX_RESIZED_NOTIFICATION,
    AX_MOVED_NOTIFICATION,
    AX_CREATED_NOTIFICATION,
    AX_SELECTED_ROWS_CHANGED_NOTIFICATION,
    AX_SELECTED_COLUMNS_CHANGED_NOTIFICATION,
    AX_SELECTED_TEXT_CHANGED_NOTIFICATION,
    AX_TITLE_CHANGED_NOTIFICATION,
    AX_LAYOUT_CHANGED_NOTIFICATION,
    AX_ANNOUNCEMENT_REQUESTED_NOTIFICATION,
];

/// Mirrors the `ApplicationServices` `kAXUIElementsKey` constant.
pub const AXUI_ELEMENTS_KEY: &str = "AXUIElementsKey";
/// Mirrors the `ApplicationServices` `kAXPriorityKey` constant.
pub const AX_PRIORITY_KEY: &str = "AXPriorityKey";
/// Mirrors the `ApplicationServices` `kAXAnnouncementKey` constant.
pub const AX_ANNOUNCEMENT_KEY: &str = "AXAnnouncementKey";
/// Mirrors the `ApplicationServices` `kAXUIElementTitleKey` constant.
pub const AXUI_ELEMENT_TITLE_KEY: &str = "AXUIElementTitleKey";

/// Collects the `ApplicationServices` AX notification-info keys exposed by this module.
pub const INFO_KEYS: &[&str] = &[
    AXUI_ELEMENTS_KEY,
    AX_PRIORITY_KEY,
    AX_ANNOUNCEMENT_KEY,
    AXUI_ELEMENT_TITLE_KEY,
];
