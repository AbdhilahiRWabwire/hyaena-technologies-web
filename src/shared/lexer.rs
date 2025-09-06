use std::{
    primitive::{bool, char},
    string::String,
    vec::Vec,
};

use crate::tokens::{
    escape_tokens::{EscapeToken, escape_tokens_vector},
    number_tokens::{NumberToken, numbers_vector},
};

// Returns True if Alphabetic Character
pub fn alphabetic_character(source_code: String) -> bool {
    return source_code.to_lowercase() != source_code.to_uppercase();
}

// Returns True if Integer
pub fn integer_character(source_code: String) -> bool {
    let character: Vec<char> = source_code.chars().collect();
    let numbers: Vec<NumberToken> = numbers_vector();

    return character[0] >= numbers[0].to_string().remove(0)
        && character[0] <= numbers[9].to_string().remove(0);
}

// Returns True if Null Character
pub fn null_character(source_code: String) -> bool {
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();

    return source_code == escape_tokens[4].to_string();
}

// Returns True if Whitespace
pub fn whitespace_character(source_code: String) -> bool {
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();

    return source_code == " ".to_string()
        || source_code == escape_tokens[3].to_string()
        || source_code == escape_tokens[6].to_string();
}
