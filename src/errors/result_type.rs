#![allow(dead_code)]

use std::primitive::bool;

// Custom Result Type Definition
pub struct Result<T> {
    error: T,
    successful: bool,
    value: T,
}
