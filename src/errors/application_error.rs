#![allow(dead_code)]

use std::string::String;

// Application Error Definition
pub struct ApplicationError<T> {
    pub error_message: String,
    pub error_value: T,
}
