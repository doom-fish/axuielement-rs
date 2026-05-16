//! Print the Accessibility trust status without triggering the system prompt.

use axuielement::prelude::*;

fn main() {
    let options = ProcessTrustOptions::with_prompt(false);
    println!("api_enabled={}", api_enabled());
    println!("is_trusted={}", is_process_trusted());
    println!(
        "is_trusted_with_options={}",
        is_process_trusted_with_options(options)
    );
}
