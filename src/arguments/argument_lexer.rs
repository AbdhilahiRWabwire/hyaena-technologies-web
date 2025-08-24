use std::{primitive::u8, string::String, vec::Vec};

use super::argument_tokenizer::ArgumentToken;

// Argument Lexer Definition
pub struct ArgumentLexer {
    pub current_line: u8,
    pub positon: u8,
    pub source: String,
    pub tokens: Vec<ArgumentToken>,
}

// Argument Lexer
pub fn argument_lexer(
    line: u8,
    pos: u8,
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

// Append an Argument Token
pub fn push(mut lexer: ArgumentLexer, token: ArgumentToken) {
    let argument_token = lexer.tokens.push(token);

    return argument_token;
}
