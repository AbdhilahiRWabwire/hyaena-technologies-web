#![allow(dead_code)]

use std::primitive::u8;

// Process Result Definition
pub type ProcessResult = u8;

// Process Result Type
pub const SUCCESFUL_RESULT: ProcessResult = 0;
pub const ERROR_RESULT: ProcessResult = 1;
