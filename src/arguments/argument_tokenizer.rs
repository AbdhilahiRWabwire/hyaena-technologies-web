use std::{
    env::args,
    process::ExitCode
};

use crate::utility::{
    print_help::print_help_message,
    print_version::print_version_number
};

// Command Line Argument Tokenizer
pub fn tokenize_arguments() -> ExitCode {
    let command_line_arguments: Vec<String> = args().collect();
    
    if command_line_arguments.len() != 2 {
        println!("Command or Flag Required but not Both: {:#?}", command_line_arguments);
        print_help_message();
        println!("Error(1) - Exiting Hyaena Technologies Web Service");
        ExitCode::FAILURE;
    } else {
        match command_line_arguments[1].trim() {
            "help" | "--h" => {
                print_help_message();
            }
            "version" | "--v" => {
                print_version_number();
            }
            &_ => {
                println!("Uknown Command or Flag: {:#?}", command_line_arguments[1].trim());
                print_help_message();
                println!("Error(1) - Exiting Hyaena Technologies Web Service");
                ExitCode::FAILURE;
            }
        };
    };

    return ExitCode::SUCCESS;
}
