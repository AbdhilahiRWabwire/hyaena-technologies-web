use std::{
    primitive::{bool, char, str, usize},
    string::String,
    vec::Vec,
};

use crate::arguments::{
    argument_lexer::{
        ArgumentLexer, advance_position, advance_to_position, current_position, end_of_file,
    },
    argument_tokenizer::ArgumentToken,
};

// Advance Position Test
#[test]
pub fn advance_position_test() -> () {}

// Advance to Position Test
#[test]
pub fn advance_to_position_test() -> () {}

// Current Position Test
#[test]
pub fn current_position_test() -> () {}

// End of Fil Test
#[test]
pub fn end_of_file_test() -> () {}
