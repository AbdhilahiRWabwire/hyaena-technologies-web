use std::{
    primitive::{bool, char},
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
pub fn tokenize(source_args: String) {
    let source: Vec<char> = source_args.chars().collect();
    let character: bool = alphabetic_character(source[0].to_string());
    let flag: bool = flag_character(source[0].to_string());
    let integer: bool = integer_character(source[0].to_string());
    let null: bool = null_character(source[0].to_string());
    let whitespace: bool = whitespace_character(source[0].to_string());
    let tokens: Vec<ArgumentToken> = Vec::new();
    let lexer: ArgumentLexer = ArgumentLexer {
        current_line: 1,
        current_positon: 0,
        source_arguments: source_args,
        argument_tokens: tokens,
    };
    let eof: bool = end_of_file(lexer);

    while source.len() > 0 {}
}
