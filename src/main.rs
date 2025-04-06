mod arguments;
use arguments::prompt_command::command_prompt;

mod configuration;
mod hypertext;
mod service;
mod utility;

// Main Entry Point
fn main() -> () {
    command_prompt();

    return ();
}
