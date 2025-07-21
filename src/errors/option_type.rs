#![allow(dead_code)]

use std::{option::Option, primitive::bool};

// Custom Option Type Definition
pub struct Optional<T> {
    null: bool,
    option_value: Option<T>,
}

// Initialize a Option with a Value
fn some<T>(value: Option<T>) -> Optional<T> {
    let option_value: Optional<T> = Optional {
        null: false,
        option_value: value,
    };

    return option_value;
}

// Return a Null Option
fn none<T>() -> Optional<T> {
    let null_option: Optional<T> = Optional {
        null: true,
        option_value: None,
    };

    return null_option;
}
