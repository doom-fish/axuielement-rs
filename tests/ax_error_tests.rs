use axuielement::AXError;

#[test]
fn error_codes_round_trip() {
    assert_eq!(AXError::Failure.raw_code(), -25_200);
    assert_eq!(AXError::APIDisabled.raw_code(), -25_211);
}

#[test]
fn localized_description_is_non_empty() {
    assert!(!AXError::CannotComplete.localized_description().is_empty());
}
