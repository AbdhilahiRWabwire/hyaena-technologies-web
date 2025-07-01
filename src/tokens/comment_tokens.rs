#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Comment Token Defintion
pub type CommentToken = &'static str;

// Comment Tokens
pub const BLOCK_COMMENT_TOKEN: CommentToken = "/*";
pub const LINE_COMMENT_TOKEN: CommentToken = "//";
pub const INNER_LINE_COMMENT_TOKEN: CommentToken = "//!";
pub const INNER_BLOCK_COMMENT_TOKEN: CommentToken = "/*!";
pub const OUTER_LINE_COMMENT_TOKEN: CommentToken = "///";
pub const OUTER_BLOCK_COMMENT_TOKEN: CommentToken = "/**";
pub const OCTOTHORPE_BLOCK_COMMENT_TOKEN: CommentToken = "#*";
pub const OCTOTHORPE_LINE_COMMENT_TOKEN: CommentToken = "#";
pub const OCTOTHROPE_INNER_LINE_COMMENT_TOKEN: CommentToken = "#!";
pub const OCTOTHORPE_INNER_BLOCK_COMMENT_TOKEN: CommentToken = "#*!";
pub const OCTOTHROPE_OUTER_LINE_COMMENT_TOKEN: CommentToken = "###";
pub const OCTOTHORPE_OUTER_BLOCK_COMMENT_TOKEN: CommentToken = "#**";

// Comment Token Vector
pub fn comments_vector() -> Vec<CommentToken> {
    let comments: Vec<CommentToken> = Vec::from([
        BLOCK_COMMENT_TOKEN,
        LINE_COMMENT_TOKEN,
        INNER_LINE_COMMENT_TOKEN,
        INNER_BLOCK_COMMENT_TOKEN,
        OUTER_LINE_COMMENT_TOKEN,
        OUTER_BLOCK_COMMENT_TOKEN,
        OCTOTHORPE_BLOCK_COMMENT_TOKEN,
        OCTOTHORPE_LINE_COMMENT_TOKEN,
        OCTOTHROPE_INNER_LINE_COMMENT_TOKEN,
        OCTOTHORPE_INNER_BLOCK_COMMENT_TOKEN,
        OCTOTHROPE_OUTER_LINE_COMMENT_TOKEN,
        OCTOTHORPE_OUTER_BLOCK_COMMENT_TOKEN,
    ]);

    return comments;
}
