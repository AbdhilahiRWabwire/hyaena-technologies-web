#![allow(dead_code)]

use std::{option::Option, primitive::bool};

// Custom Result Type Definition
pub struct Resultant<T> {
    error: bool,
    error_value: Option<T>,
    result_value: Option<T>,
}

// Initialize a Result with a Value
fn okay<T>(value: Option<T>) -> Resultant<T> {
    let result_value: Resultant<T> = Resultant {
        error: false,
        error_value: None,
        result_value: value,
    };

    return result_value;
}

// Initialize a Result with a Error
fn error<T>(error: Option<T>) -> Resultant<T> {
    let error_value: Resultant<T> = Resultant {
        error: true,
        error_value: error,
        result_value: None,
    };

    return error_value;
}
