use std::{
    env::args,
    io::{StdoutLock, Write, stdout},
    primitive::{bool, char, usize},
    process::exit,
    str::Lines,
    string::String,
    vec::Vec,
};

use crate::tokens::{
    operator_tokens::{OperatorToken, operators_vector},
    token_type::TokenType,
};

use super::print_help::print_help_message;

// Argument Token Definition
pub struct ArgumentToken {
    pub lexeme: String,
    pub token_type: TokenType,
}

// Argument Lexer Definition
pub struct ArgumentLexer {
    pub argument_lines: Lines<'static>,
    pub argument_source: &'static String,
    pub argument_tokens: Vec<ArgumentToken>,
    pub current_positon: usize,
}

// Advance Lexer Position by 1
pub fn advance_position(argument_lexer: &ArgumentLexer) -> usize {
    let position: usize = argument_lexer.current_positon;

    return position + 1;
}

// Advance Lexer to Position
pub fn advance_to_position(argument_lexer: &ArgumentLexer, index: usize) -> usize {
    let position: usize = argument_lexer.current_positon;

    return position + index;
}

// Lexer Current Position
pub fn current_position(argument_lexer: &ArgumentLexer) -> char {
    let position: usize = argument_lexer.current_positon;
    let source: Vec<char> = argument_lexer.argument_source.chars().collect();

    return source[position];
}

// Returns True if Flag Character
pub fn flag_character(source_arguments: String) -> bool {
    let operators: Vec<OperatorToken> = operators_vector();

    if source_arguments.starts_with(operators[39]) || source_arguments.starts_with(operators[11]) {
        return true;
    } else {
        return false;
    }
}

// Returns True if Lexer Position is at End of File
pub fn end_of_file(argument_lexer: &ArgumentLexer) -> bool {
    let position: usize = argument_lexer.current_positon;
    let source: &'static String = argument_lexer.argument_source;

    return position >= source.len();
}

// Print Argument Token
pub fn print_argument_token(argument_token: &ArgumentToken) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", argument_token.lexeme).unwrap();
    writeln!(standard_output, "{}", argument_token.token_type).unwrap();

    return ();
}

// Unknow Argument Error
pub fn unknown_argument() -> () {
    let mut standard_output: StdoutLock = stdout().lock();
    let arguments: Vec<String> = args().collect();

    writeln!(standard_output, "Uknown Command or Flag: {}", arguments[1]).unwrap();
    print_help_message();
    writeln!(
        standard_output,
        "Error(1) - Exiting Hyaena Technologies Web Service"
    )
    .unwrap();
    exit(1)
}
