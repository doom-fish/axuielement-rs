use super::Handle;

extern "C" {
    pub fn ax_text_marker_get_type_id() -> usize;
    pub fn ax_text_marker_create(bytes: *const u8, length: usize) -> Handle;
    pub fn ax_text_marker_retain(handle: Handle) -> Handle;
    pub fn ax_text_marker_release(handle: Handle);
    pub fn ax_text_marker_len(handle: Handle) -> usize;
    pub fn ax_text_marker_copy_bytes(handle: Handle, buffer: *mut u8, capacity: usize) -> bool;

    pub fn ax_text_marker_range_get_type_id() -> usize;
    pub fn ax_text_marker_range_create(start_handle: Handle, end_handle: Handle) -> Handle;
    pub fn ax_text_marker_range_create_with_bytes(
        start_bytes: *const u8,
        start_length: usize,
        end_bytes: *const u8,
        end_length: usize,
    ) -> Handle;
    pub fn ax_text_marker_range_retain(handle: Handle) -> Handle;
    pub fn ax_text_marker_range_release(handle: Handle);
    pub fn ax_text_marker_range_copy_start_marker(handle: Handle) -> Handle;
    pub fn ax_text_marker_range_copy_end_marker(handle: Handle) -> Handle;
}
