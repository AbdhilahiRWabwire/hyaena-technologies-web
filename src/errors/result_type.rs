#![allow(dead_code)]

use std::{option::Option, primitive::bool};

// Custom Result Type Definition
pub struct Result<T> {
    error: Option<T>,
    successful: bool,
    value: Option<T>,
}

// Initialize a Result with a Validated Value
fn okay<T>(val: Option<T>) -> Result<T> {
    let result_value: Result<T> = Result {
        error: None,
        successful: true,
        value: val,
    };

    return result_value;
}

// Initialize a Result with a Validated Error
fn error<T>(err: Option<T>) -> Result<T> {
    let result_error: Result<T> = Result {
        error: err,
        successful: false,
        value: None,
    };

    return result_error;
}
