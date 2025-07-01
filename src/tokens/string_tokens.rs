#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// String Token Defintion
pub type StringToken = &'static str;

// String Tokens
pub const APOSTROPHE_TOKEN: StringToken = "\'";
pub const GRAVE_ACCENT_TOKEN: StringToken = "/`";
pub const QUOTATION_MARK_TOKEN: StringToken = "\"";

// String Token Vector
pub fn strings_vector() -> Vec<StringToken> {
    let strings: Vec<StringToken> =
        Vec::from([APOSTROPHE_TOKEN, GRAVE_ACCENT_TOKEN, QUOTATION_MARK_TOKEN]);

    return strings;
}
