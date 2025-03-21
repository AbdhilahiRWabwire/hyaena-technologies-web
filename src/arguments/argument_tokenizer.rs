use std::env::args;

use crate::utility::{
    exit_program::{error_exit, successful_exit}, 
    print_help::print_help_message,
    print_version::print_version_number
};

// Command Line Argument Tokenizer
pub fn tokenize_arguments() -> () {
    let command_line_arguments: Vec<String> = args().collect();
    
    if command_line_arguments.len() != 2 {
        println!("Command or Flag Required but not Both: {:#?}", command_line_arguments);
        print_help_message();
        error_exit();
    } else {
        match command_line_arguments[1].trim() {
            "exit" => successful_exit(),
            "help" => print_help_message(),
            "version" => print_version_number(),
            "--help" => print_help_message(),
            "--h" => print_help_message(),
            "--version" => print_version_number(),
            "--v" => print_version_number(),
            &_ => error_exit()
        }
    }

    return ();
}
