mod arguments;
use arguments::command_flag::print_help_message;

mod networking;
mod yaml;
mod utility;

// Main Entry Point
fn main() -> () {
    print_help_message();
    return ();
}
