use std::{
    primitive::{bool, char},
    str::Lines,
    string::String,
    vec::Vec,
};

use crate::{
    shared::lexer::{
        alphabetic_character, integer_character, null_character, whitespace_character,
    },
    tokens::token_type::TokenType,
};

use super::argument_lexer::{
    ArgumentLexer, ArgumentToken, advance_position, advance_to_position, current_position,
    end_of_file, flag_character, print_argument_token, unknown_argument,
};

// Tokenize Command Line Arguments
pub fn tokenize(source: &'static String) {
    let lines: Lines<'static> = source.lines();
    let tokens: Vec<ArgumentToken> = Vec::new();
    let lexer: ArgumentLexer = ArgumentLexer {
        argument_lines: lines,
        argument_source: source,
        argument_tokens: tokens,
        current_positon: 0,
    };
    let advance_lexer: usize = advance_position(&lexer);
    let characters: Vec<char> = lexer.argument_source.chars().collect();
    let character: bool = alphabetic_character(characters[0].to_string());
    let current_character: char = current_position(&lexer);
    let eof: bool = end_of_file(&lexer);
    let flag: bool = flag_character(characters[0].to_string());
    let integer: bool = integer_character(characters[0].to_string());
    let null: bool = null_character(characters[0].to_string());
    let whitespace: bool = whitespace_character(characters[0].to_string());

    while characters.len() > 0 {}
}
