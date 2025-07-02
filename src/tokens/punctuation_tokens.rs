#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Punctuation Token Defintion
pub type PunctuationToken = &'static str;

//  Tokens
pub const APOSTROPHE_TOKEN: PunctuationToken = "\'";
pub const COLON_TOKEN: PunctuationToken = ":";
pub const COMMA_TOKEN: PunctuationToken = ",";
pub const GRAVE_ACCENT_TOKEN: PunctuationToken = "/`";
pub const QUOTATION_MARK_TOKEN: PunctuationToken = "\"";

// String Token Vector
pub fn strings_vector() -> Vec<PunctuationToken> {
    let strings: Vec<PunctuationToken> = Vec::from([
        APOSTROPHE_TOKEN,
        COLON_TOKEN,
        COMMA_TOKEN,
        GRAVE_ACCENT_TOKEN,
        QUOTATION_MARK_TOKEN,
    ]);

    return strings;
}
