use std::{primitive::bool, vec::Vec};

use crate::{
    arguments::argument_lexer::flag_character,
    tokens::{
        character_tokens::{CharacterToken, characters_vector},
        escape_tokens::{EscapeToken, escape_tokens_vector},
        number_tokens::{NumberToken, numbers_vector},
        operator_tokens::{OperatorToken, operators_vector},
    },
};

// Flag Character Test
#[test]
pub fn flag_character_test() -> () {
    let character_tokens: Vec<CharacterToken> = characters_vector();
    let escape_tokens: Vec<EscapeToken> = escape_tokens_vector();
    let lowercase_character: bool = flag_character(character_tokens[0].to_string());
    let null_character: bool = flag_character(escape_tokens[4].to_string());
    let number_tokens: Vec<NumberToken> = numbers_vector();
    let number: bool = flag_character(number_tokens[4].to_string());
    let operator_tokens: Vec<OperatorToken> = operators_vector();
    let decrement_character: bool = flag_character(operator_tokens[11].to_string());
    let subtraction_character: bool = flag_character(operator_tokens[39].to_string());
    let uppercase_character: bool = flag_character(character_tokens[26].to_string());
    let whitespace_character: bool = flag_character(" ".to_string());

    assert_eq!(decrement_character, true);
    assert_eq!(null_character, false);
    assert_eq!(number, false);
    assert_eq!(lowercase_character, false);
    assert_eq!(subtraction_character, true);
    assert_eq!(uppercase_character, false);
    assert_eq!(whitespace_character, false);
}
