use std::env::args;
use std::fmt;

use super::commands::comand_map;

use super::flags::flag_map;

// Command Line Argument Tokenizer
pub fn tokenize_arguments() {
    let command_line_arguments: Args = args();

    if command_line_arguments < 2 {
        println!("Command or Flag Required");
    }
}
