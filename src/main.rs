mod arguments;
use std::process::ExitCode;

use arguments::prompt_command::command_prompt;

mod networking;
mod yaml;
mod utility;

// Main Entry Point
fn main() -> ExitCode {
    command_prompt();
    return ExitCode::SUCCESS;
}
