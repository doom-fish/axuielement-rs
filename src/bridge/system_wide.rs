use super::{AXStatus, Handle};

extern "C" {
    pub fn ax_system_wide_create() -> Handle;
    pub fn ax_system_wide_copy_focused_application(
        handle: Handle,
        out_element: *mut Handle,
    ) -> AXStatus;
    pub fn ax_system_wide_copy_focused_ui_element(
        handle: Handle,
        out_element: *mut Handle,
    ) -> AXStatus;
    pub fn ax_system_wide_copy_focused_window(handle: Handle, out_element: *mut Handle)
        -> AXStatus;
}
