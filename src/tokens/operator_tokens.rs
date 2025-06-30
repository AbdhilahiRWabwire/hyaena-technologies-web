#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Operator Token Defintion
pub type OperatorToken = &'static str;

// Operator Tokens
pub const ADDITION_TOKEN: OperatorToken = "+";
pub const ADDITION_ASSIGNMENT_TOKEN: OperatorToken = "+=";
pub const AND_TOKEN: OperatorToken = "&&";
pub const ASSIGNMENT_TOKEN: OperatorToken = "=";
pub const DIVISION_TOKEN: OperatorToken = "/";
pub const DIVISION_ASSIGNMENT_TOKEN: OperatorToken = "/=";
pub const EQUALITY_TOKEN: OperatorToken = "==";
pub const GREATER_THAN_TOKEN: OperatorToken = ">";
pub const GREATER_OR_EQUAL_TOKEN: OperatorToken = ">=";
pub const LESS_THAN_TOKEN: OperatorToken = "<";
pub const LESS_OR_EQUAL_TOKEN: OperatorToken = "<=";
pub const MODULUS_TOKEN: OperatorToken = "%";
pub const MODULUS_ASSIGNMENT_TOKEN: OperatorToken = "%=";
pub const MULTIPLICATION_TOKEN: OperatorToken = "*";
pub const MULTIPLICATION_ASSIGNMENT_TOKEN: OperatorToken = "*=";
pub const NOT_TOKEN: OperatorToken = "!";
pub const NOT_EQUAL_TOKEN: OperatorToken = "!=";
pub const NULL_COALESCING_TOKEN: OperatorToken = "?";
pub const OR_TOKEN: OperatorToken = "||";
pub const SUBTRACTION_TOKEN: OperatorToken = "-";
pub const SUBTRACTION_ASSIGNMENT_TOKEN: OperatorToken = "-=";
pub const RANGE_TOKEN: OperatorToken = "..";

// Operator Token Vector
pub fn operators_vector() -> Vec<OperatorToken> {
    let operators: Vec<OperatorToken> = Vec::from([
        ADDITION_TOKEN,
        ADDITION_ASSIGNMENT_TOKEN,
        AND_TOKEN,
        ASSIGNMENT_TOKEN,
        DIVISION_TOKEN,
        DIVISION_ASSIGNMENT_TOKEN,
        EQUALITY_TOKEN,
        GREATER_THAN_TOKEN,
        GREATER_OR_EQUAL_TOKEN,
        LESS_THAN_TOKEN,
        LESS_OR_EQUAL_TOKEN,
        MODULUS_TOKEN,
        MODULUS_ASSIGNMENT_TOKEN,
        MULTIPLICATION_TOKEN,
        MULTIPLICATION_ASSIGNMENT_TOKEN,
        NOT_TOKEN,
        NOT_EQUAL_TOKEN,
        NULL_COALESCING_TOKEN,
        OR_TOKEN,
        SUBTRACTION_TOKEN,
        SUBTRACTION_ASSIGNMENT_TOKEN,
        RANGE_TOKEN,
    ]);

    return operators;
}
