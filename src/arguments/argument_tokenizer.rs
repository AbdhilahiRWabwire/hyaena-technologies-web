use std::env::Args;
use std::env::args;

// Command Line Argument Tokenizer
pub fn tokenize_arguments() {
    let command_line_arguments: Args = args();

    if command_line_arguments.count() < 2 {
        println!("Command or Flag Required");
    }
}
