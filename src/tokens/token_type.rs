#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Token Type Defintion
pub type TokenType = &'static str;

// Token Types
pub const CHARACTER_TOKEN: TokenType = "CHARACTER";
pub const COMMENT_TOKEN: TokenType = "COMMENT";
pub const DELIMITER_TOKEN: TokenType = "DELIMITER";
pub const KEYWORD_TOKEN: TokenType = "KEYWORD";
pub const NUMBER_TOKEN: TokenType = "NUMBER";
pub const OPERATOR_TOKEN: TokenType = "OPERATOR";
pub const STRING_TOKEN: TokenType = "STRING";

// Token Type Vector
pub fn token_types_vector() -> Vec<TokenType> {
    let token_types: Vec<TokenType> = Vec::from([
        CHARACTER_TOKEN,
        COMMENT_TOKEN,
        DELIMITER_TOKEN,
        KEYWORD_TOKEN,
        NUMBER_TOKEN,
        OPERATOR_TOKEN,
        STRING_TOKEN,
    ]);

    return token_types;
}
