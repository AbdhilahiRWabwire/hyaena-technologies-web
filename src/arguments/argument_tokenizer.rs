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

use super::{
    argument_lexer::alphabetic_character, print_help::print_help_message,
    print_version::print_version_number,
};

use crate::tokens::{
    operator_tokens::{OperatorToken, operators_vector},
    token_type::{COMMAND_TOKEN, FLAG_TOKEN, TokenType, token_types_vector},
};

// Argument Token Definition
pub struct ArgumentToken {
    pub value: String,
    pub token_type: TokenType,
}

// Print Argument Token
pub fn print_argument_token(token: ArgumentToken) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", token.value).unwrap();
    writeln!(standard_output, "{}", token.token_type).unwrap();

    return ();
}

// Return Argument Token Type
pub fn argument_token_type(arg_type: Vec<TokenType>) -> TokenType {
    match arg_type.as_slice() {
        [COMMAND_TOKEN] => {
            return arg_type[2];
        }

        [FLAG_TOKEN] => {
            return arg_type[4];
        }

        _ => {
            return arg_type[10];
        }
    }
}

// Return Argument Token
pub fn argument_token(arg_value: String, arg_type: TokenType) -> ArgumentToken {
    let arg_token: ArgumentToken = ArgumentToken {
        value: arg_value,
        token_type: arg_type,
    };

    return arg_token;
}

// Tokenize Command Line Arguments
pub fn tokenize() -> Vec<ArgumentToken> {
    let mut argument_tokens: Vec<ArgumentToken> = Vec::new();
    let mut arguments: Vec<String> = args().collect();
    let character: bool = alphabetic_character(arguments[1].remove(0).to_string());
    let operators: Vec<OperatorToken> = operators_vector();
    let token_types: Vec<TokenType> = token_types_vector();

    if arguments[1].starts_with(operators[11]) || arguments[1].starts_with(operators[42]) {
        argument_tokens.push(argument_token(arguments[1].clone(), token_types[2]));
    } else if character == true {
        argument_tokens.push(argument_token(arguments[1].clone(), token_types[4]));
    };

    return argument_tokens;
}

// Command Line Argument Tokenizer
pub fn tokenize_arguments() -> ExitCode {
    let mut standard_output: StdoutLock = stdout().lock();
    let command_line_arguments: Vec<String> = args().collect();

    if command_line_arguments.len() != 2 {
        writeln!(
            standard_output,
            "Command or Flag Required but not Both: {:#?}",
            command_line_arguments
        )
        .unwrap();
        print_help_message();
        writeln!(
            standard_output,
            "Error(1) - Exiting Hyaena Technologies Web Service"
        )
        .unwrap();
        return ExitCode::FAILURE;
    } else {
        match command_line_arguments[1].trim() {
            "help" | "--h" => {
                print_help_message();
            }
            "version" | "--v" => {
                print_version_number();
            }
            &_ => {
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
                return ExitCode::FAILURE;
            }
        };
    };

    return ExitCode::SUCCESS;
}
