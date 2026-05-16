use axuielement::process_trust::{
    api_enabled, is_process_trusted, is_process_trusted_with_options, ProcessTrustOptions,
};

#[test]
fn trust_queries_return_bools() {
    let _ = api_enabled();
    let _ = is_process_trusted();
    let _ = is_process_trusted_with_options(ProcessTrustOptions::with_prompt(false));
}

#[test]
fn process_trust_options_builder_sets_prompt() {
    assert!(ProcessTrustOptions::with_prompt(true).prompt);
    assert!(!ProcessTrustOptions::new().prompt);
}
