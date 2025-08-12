use std::{
    env::args,
    io::{BufReader, Error, Read, StdoutLock, Write, stdout},
    option::Option,
    path::PathBuf,
    primitive::str,
    process::{ExitCode, exit},
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
    vec::Vec,
};

use super::{print_help::print_help_message, print_version::print_version_number};
use crate::tokens::token_type::{COMMAND_TOKEN, FLAG_TOKEN, TokenType};

// Argument Token Definition
pub struct ArgumentToken {
    pub value: &'static str,
    pub token_type: TokenType,
}

// Flag Token Definition
pub type FlagToken = &'static str;

// Flag Tokens
pub const DOUBLE_DASH: FlagToken = "--";
pub const SINGLE_DASH: FlagToken = "-";

// Print Argument Token
pub fn print_token(token: ArgumentToken) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", token.value).unwrap();
    writeln!(standard_output, "{}", token.token_type).unwrap();

    return ();
}

// Return Token Type
pub fn token_type(arg_type: Vec<TokenType>) -> TokenType {
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
pub fn argument_token(arg_value: &'static str, arg_type: TokenType) -> ArgumentToken {
    let argument_token: ArgumentToken = ArgumentToken {
        value: arg_value,
        token_type: arg_type,
    };

    return argument_token;
}

// Tokenize Command Line Arguments
pub fn tokenize(token_source: &'static String) -> Vec<ArgumentToken> {
    let mut argument_tokens: Vec<ArgumentToken> = Vec::new();
    let mut source_tokens: Vec<&str> = token_source.split(" ").collect();

    while source_tokens.len() > 0 {
        if source_tokens[0] == DOUBLE_DASH || source_tokens[0] == SINGLE_DASH {
            argument_tokens.push(token(source_tokens.remove(0), FLAG_TOKEN));
        }
    }

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
