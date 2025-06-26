#![allow(dead_code)]

use std::primitive::u8;

// Process Option Definition
pub type ProcessOption = u8;

// Process Option Type
pub const NULL_OPTION: ProcessOption = 0;
pub const OPTION_VALUE: ProcessOption = 1;
