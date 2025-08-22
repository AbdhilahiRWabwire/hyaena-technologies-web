use std::{primitive::bool, vec::Vec};

use crate::arguments::argument_lexer::{
    alphabetic_character, integer_character, null_character, whitespace_character,
};
use crate::tokens::{
    character_tokens::{CharacterToken, characters_vector},
    escape_tokens::{EscapeToken, escape_tokens_vector},
    number_tokens::{NumberToken, numbers_vector},
};

// Alphabetic Character Test
#[test]
fn alphabetic_character_test() -> () {
    let character_tokens: Vec<CharacterToken> = characters_vector();
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();
    let lowercase_character: bool = alphabetic_character(character_tokens[0].to_string());
    let null_character: bool = alphabetic_character(escape_tokens[4].to_string());
    let number_tokens: Vec<NumberToken> = numbers_vector();
    let number: bool = alphabetic_character(number_tokens[4].to_string());
    let uppercase_character: bool = alphabetic_character(character_tokens[26].to_string());
    let whitespace_character: bool = alphabetic_character(" ".to_string());

    assert_eq!(null_character, false);
    assert_eq!(number, false);
    assert_eq!(lowercase_character, true);
    assert_eq!(uppercase_character, true);
    assert_eq!(whitespace_character, false);
}

// Integer Character Test
#[test]
fn integer_character_test() -> () {
    let character_tokens: Vec<CharacterToken> = characters_vector();
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();
    let lowercase_character: bool = integer_character(character_tokens[0].to_string());
    let null_character: bool = integer_character(escape_tokens[4].to_string());
    let number_tokens: Vec<NumberToken> = numbers_vector();
    let number: bool = integer_character(number_tokens[4].to_string());
    let uppercase_character: bool = integer_character(character_tokens[26].to_string());
    let whitespace_character: bool = integer_character(" ".to_string());

    assert_eq!(null_character, false);
    assert_eq!(number, true);
    assert_eq!(lowercase_character, false);
    assert_eq!(uppercase_character, false);
    assert_eq!(whitespace_character, false);
}

// Null Character Test
#[test]
fn null_character_test() -> () {
    let character_tokens: Vec<CharacterToken> = characters_vector();
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();
    let lowercase_character: bool = null_character(character_tokens[0].to_string());
    let null: bool = null_character(escape_tokens[4].to_string());
    let number_tokens: Vec<NumberToken> = numbers_vector();
    let number: bool = null_character(number_tokens[4].to_string());
    let uppercase_character: bool = null_character(character_tokens[26].to_string());
    let whitespace_character: bool = null_character(" ".to_string());

    assert_eq!(null, true);
    assert_eq!(number, false);
    assert_eq!(lowercase_character, false);
    assert_eq!(uppercase_character, false);
    assert_eq!(whitespace_character, false);
}

// Whitespace Character Test
#[test]
fn whitespace_character_test() -> () {
    let character_tokens: Vec<CharacterToken> = characters_vector();
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();
    let lowercase_character: bool = whitespace_character(character_tokens[0].to_string());
    let null_character: bool = whitespace_character(escape_tokens[4].to_string());
    let number_tokens: Vec<NumberToken> = numbers_vector();
    let number: bool = whitespace_character(number_tokens[4].to_string());
    let uppercase_character: bool = whitespace_character(character_tokens[26].to_string());
    let whitespace: bool = whitespace_character(" ".to_string());

    assert_eq!(null_character, false);
    assert_eq!(number, false);
    assert_eq!(lowercase_character, false);
    assert_eq!(uppercase_character, false);
    assert_eq!(whitespace, true);
}
