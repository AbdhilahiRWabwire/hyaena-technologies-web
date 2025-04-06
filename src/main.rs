use std::process::ExitCode;

mod arguments;
use arguments::prompt_command::command_prompt;

mod configuration;
mod hypertext;
mod service;
mod utility;

// Main Entry Point
fn main() -> ExitCode {
    command_prompt();

    return ExitCode::SUCCESS;
}
