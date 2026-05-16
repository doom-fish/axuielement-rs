use core::ffi::c_char;

extern "C" {
    pub fn ax_process_trust_api_enabled() -> bool;
    pub fn ax_process_trust_is_trusted() -> bool;
    pub fn ax_process_trust_is_trusted_with_prompt(prompt: bool) -> bool;
    pub fn ax_process_trust_make_process_trusted(executable_path: *const c_char) -> i32;
}
