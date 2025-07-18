#![allow(dead_code)]

use std::primitive::bool;

// Custom Option Type Definition
pub struct Option<T> {
    null: bool,
    value: T,
}
