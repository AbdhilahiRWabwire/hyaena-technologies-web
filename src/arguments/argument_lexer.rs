use std::{
    env::args,
    io::{StdoutLock, Write, stdout},
    primitive::{bool, char, u8, usize},
    process::exit,
    string::String,
    vec::Vec,
};

use crate::tokens::{
    operator_tokens::{OperatorToken, operators_vector},
    token_type::TokenType,
};

use super::print_help::print_help_message;

// Argument Lexer Definition
pub struct ArgumentLexer {
    pub current_line: u8,
    pub positon: usize,
    pub source: String,
    pub tokens: Vec<ArgumentToken>,
}

// Argument Token Definition
pub struct ArgumentToken {
    pub lexeme: String,
    pub token_type: TokenType,
}

// Advance Lexer Position by 1
pub fn advance_position(lexer: ArgumentLexer) -> usize {
    let position: usize = lexer.positon;

    return position + 1;
}

// Advance Lexer to Position
pub fn advance_to_position(lexer: ArgumentLexer, index: usize) -> usize {
    let position: usize = lexer.positon;

    return position + index;
}

// Lexer Current Position
pub fn current_position(lexer: ArgumentLexer) -> char {
    let position: usize = lexer.positon;
    let source: Vec<char> = lexer.source.chars().collect();

    return source[position];
}

// Returns True if Flag Character
pub fn flag_character(source: String) -> bool {
    let operators: Vec<OperatorToken> = operators_vector();

    if source.starts_with(operators[39]) || source.starts_with(operators[11]) {
        return true;
    } else {
        return false;
    }
}

// Returns True if Lexer Position is at End of File
pub fn end_of_file(lexer: ArgumentLexer) -> bool {
    let position: usize = lexer.positon;
    let source: String = lexer.source;

    return position >= source.len();
}

// Print Argument Token
pub fn print_argument_token(token: ArgumentToken) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", token.lexeme).unwrap();
    writeln!(standard_output, "{}", token.token_type).unwrap();

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
