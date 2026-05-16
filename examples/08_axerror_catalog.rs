//! Print a few representative AX errors and their bridge-provided descriptions.

use axuielement::AXError;

fn main() {
    for error in [
        AXError::Failure,
        AXError::CannotComplete,
        AXError::APIDisabled,
    ] {
        println!("{} => {}", error, error.localized_description());
    }
}
