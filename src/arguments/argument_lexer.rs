use std::{primitive::u8, string::String};

use super::argument_tokenizer::ArgumentToken;

// Argument Lexer Definition
pub struct ArgumentLexer {
    pub current_line: u8,
    pub positon: u8,
    pub source: String,
    pub token: ArgumentToken,
}
