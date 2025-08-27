use std::{
    primitive::{bool, char, u8, usize},
    string::String,
    vec::Vec,
};

use super::argument_tokenizer::ArgumentToken;

// Argument Lexer Definition
pub struct ArgumentLexer {
    pub current_line: u8,
    pub positon: usize,
    pub source: String,
    pub tokens: Vec<ArgumentToken>,
}

// Argument Lexer
pub fn argument_lexer(
    line: u8,
    pos: usize,
    input: String,
    chars: Vec<ArgumentToken>,
) -> ArgumentLexer {
    let lexer: ArgumentLexer = ArgumentLexer {
        current_line: line,
        positon: pos,
        source: input,
        tokens: chars,
    };

    return lexer;
}

// Lexer Advance Position
pub fn advance_position(lexer: ArgumentLexer) -> usize {
    let position: usize = lexer.positon;

    return position + 1;
}

// Lexer Advance to Position
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

// Returns True if Lexer Position at End of File
pub fn end_of_file(lexer: ArgumentLexer) -> bool {
    let position: usize = lexer.positon;
    let source: String = lexer.source;

    return position >= source.len();
}

// Append an Argument Token
pub fn push(mut lexer: ArgumentLexer, token: ArgumentToken) -> () {
    let argument_token: () = lexer.tokens.push(token);

    return argument_token;
}
