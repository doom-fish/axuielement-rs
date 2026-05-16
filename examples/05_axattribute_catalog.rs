//! Print the generated attribute, role, and value constant counts.

fn main() {
    println!(
        "attributes={} parameterized={} roles={} subroles={} values={}",
        axuielement::ax_attribute::attributes::ALL_ATTRIBUTES.len(),
        axuielement::ax_attribute::parameterized::ALL_PARAMETERIZED_ATTRIBUTES.len(),
        axuielement::ax_attribute::roles::ALL_ROLES.len(),
        axuielement::ax_attribute::subroles::ALL_SUBROLES.len(),
        axuielement::ax_attribute::values::ALL_VALUE_CONSTANTS.len(),
    );
    println!(
        "role={} focused={}",
        axuielement::ax_attribute::AX_ROLE_ATTRIBUTE,
        axuielement::ax_attribute::AX_FOCUSED_ATTRIBUTE,
    );
}
