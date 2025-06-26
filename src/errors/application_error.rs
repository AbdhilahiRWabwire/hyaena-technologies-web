#![allow(dead_code)]

use std::string::String;

// Application Error Definition
pub struct ApplicationError<T> {
    pub current_time: String,
    pub error_message: String,
    pub error_value: T,
}

// Application Error
pub fn app_error<T>(application_error: ApplicationError<T>) {}
