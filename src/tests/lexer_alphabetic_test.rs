use std::primitive::bool;

use crate::arguments::argument_lexer::alphabetic_character;
use crate::tokens::{
    character_tokens::{CharacterToken, characters_vector},
    escape_tokens::{EscapeToken, escape_tokens_vector},
    number_tokens::{NumberToken, numbers_vector},
};

// Returns False if Integer
#[test]
fn integer() -> () {
    let numbers: Vec<NumberToken> = numbers_vector();
    let character: bool = alphabetic_character(numbers[4].to_string());

    assert_eq!(character, false)
}

// Returns True if Lower Case Character
#[test]
fn lower_case_character() -> () {
    let character_tokens: Vec<CharacterToken> = characters_vector();
    let character: bool = alphabetic_character(character_tokens[0].to_string());

    assert_eq!(character, true)
}

// Returns True if Upper Case Character
#[test]
fn upper_case_character() -> () {
    let character_tokens: Vec<CharacterToken> = characters_vector();
    let character: bool = alphabetic_character(character_tokens[26].to_string());

    assert_eq!(character, true)
}

// Returns False if Null Character
#[test]
fn null_character() -> () {
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();
    let character: bool = alphabetic_character(escape_tokens[4].to_string());

    assert_eq!(character, false)
}

// Returns False if Whitespace Character
#[test]
fn whitespace_character() -> () {
    let character: bool = alphabetic_character(" ".to_string());

    assert_eq!(character, false)
}
