#![allow(dead_code)]

use std::{
    primitive::{bool, char},
    string::String,
    vec::Vec,
};

use crate::tokens::{
    escape_tokens::{EscapeToken, escape_tokens_vector},
    number_tokens::{NumberToken, numbers_vector},
};

struct ArgumentLexer {}

// Returns True if Alphabetic Character
pub fn alphabetic_character(source: String) -> bool {
    return source.to_lowercase() != source.to_uppercase();
}

// Returns True if Whitespace
pub fn whitespace(source: String) -> bool {
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();

    return source == " " || source == escape_tokens[3] || source == escape_tokens[6];
}

// Returns True if Integer
pub fn integer(mut source: String) -> bool {
    let character: char = source.remove(0);
    let numbers: Vec<NumberToken> = numbers_vector();

    return character >= numbers[0].to_string().remove(0)
        && character <= numbers[9].to_string().remove(0);
}
