#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::primitive::str;

use super::html_element::HypertextMarkupElement;

pub type HTMLTypeDeclaration = &'static str;

// Hypertext Markup Language Document Type Declaration Definition
pub const HTML_DOCUMENT_TYPE_DECLARATION: HTMLTypeDeclaration = "<!DOCTYPE HTML>";

// Hypertext Markup Language Document Definition
pub struct HypertextMarkupDocument {
    pub elements: Vec<&'static HypertextMarkupElement>,
    pub type_declaration: HTMLTypeDeclaration,
}
