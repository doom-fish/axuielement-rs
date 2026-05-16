use super::{AXStatus, Handle};

extern "C" {
    pub fn ax_ui_element_get_type_id() -> usize;
    pub fn ax_ui_element_create_application(pid: i32) -> Handle;
    pub fn ax_ui_element_retain(handle: Handle) -> Handle;
    pub fn ax_ui_element_release(handle: Handle);
    pub fn ax_ui_element_get_pid(handle: Handle, out_pid: *mut i32) -> AXStatus;
    pub fn ax_ui_element_set_messaging_timeout(handle: Handle, timeout: f32) -> AXStatus;
    pub fn ax_ui_element_copy_element_at_position(
        handle: Handle,
        x: f32,
        y: f32,
        out_element: *mut Handle,
    ) -> AXStatus;
    pub fn ax_ui_element_post_keyboard_event(
        handle: Handle,
        key_char: u16,
        virtual_key: u16,
        key_down: bool,
    ) -> AXStatus;
}
