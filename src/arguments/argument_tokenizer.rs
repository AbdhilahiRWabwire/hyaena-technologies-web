use std::{
    env::{Args, args},
    io::{BufReader, Error, Read, StdoutLock, Write, stdout},
    option::Option,
    path::PathBuf,
    primitive::{bool, str},
    process::{ExitCode, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
    vec::Vec,
};

use super::{print_help::print_help_message, print_version::print_version_number};

use crate::tokens::{
    character_tokens::{CharacterToken, characters_vector},
    operator_tokens::{OperatorToken, operators_vector},
    token_type::{COMMAND_TOKEN, FLAG_TOKEN, TokenType, token_types_vector},
};

// Argument Token Definition
pub struct ArgumentToken {
    pub lexeme: String,
    pub token_type: TokenType,
}

// Argument Token
pub fn argument_token(arg_value: String, arg_type: TokenType) -> ArgumentToken {
    let arg_token: ArgumentToken = ArgumentToken {
        lexeme: arg_value,
        token_type: arg_type,
    };

    return arg_token;
}

// Argument Token Type
pub fn argument_token_type(arg_type: Vec<TokenType>) -> TokenType {
    match arg_type.as_slice() {
        [COMMAND_TOKEN] => {
            return arg_type[1];
        }

        [FLAG_TOKEN] => {
            return arg_type[5];
        }

        _ => {
            return arg_type[9];
        }
    }
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
    let command_line_arguments: Vec<String> = args().collect();

    writeln!(
        standard_output,
        "Uknown Command or Flag: {}",
        command_line_arguments[1]
    )
    .unwrap();
    print_help_message();
    writeln!(
        standard_output,
        "Error(1) - Exiting Hyaena Technologies Web Service"
    )
    .unwrap();
    exit(1)
}

// Tokenize Command Line Arguments
pub fn tokenize() -> Vec<ArgumentToken> {
    let mut argument_tokens: Vec<ArgumentToken> = Vec::new();
    let arguments: Vec<String> = args().collect();
    let character_tokens: Vec<CharacterToken> = characters_vector();
    let operators: Vec<OperatorToken> = operators_vector();
    let token_types: Vec<TokenType> = token_types_vector();

    for token in character_tokens {
        if arguments[1].starts_with(token) {
            argument_tokens.push(argument_token(arguments[1].clone(), token_types[1]));
        } else if arguments[1].starts_with(operators[11]) || arguments[1].starts_with(operators[39])
        {
            argument_tokens.push(argument_token(arguments[1].clone(), token_types[5]));
        } else {
            unknown_argument();
        }
    }

    return argument_tokens;
}
