use super::Handle;

extern "C" {
    pub fn ax_error_copy_description(code: i32) -> Handle;
}
