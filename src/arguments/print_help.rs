use std::collections::HashMap;

use super::commands::command_map;
use super::commands::CommandArgument;
use super::commands::command_map;
use super::flags::FlagArgument;
use super::flags::flag_map;

// Print Help Command Output
pub fn print_help_message() -> () {
    let mut commands: HashMap<String, CommandArgument> = command_map();

    let mut flags: HashMap<String, FlagArgument> = flag_map();

    println!("Hyaena Technologies Web Service");
    println!("");
    println!("");
    println!("Commands:					Description:");
    println!("");

    for (command_argumen) in commands.values() {
        println!("{:#?}: {:#?}", command_argument.name, command_argument.description);
    }

    println!("");
    println!("Flags:				    Description:");
    println!("");

    for (flag_argument) in flags.values() {
        println!("{:#?}: {:#?}", flag_argument.name, flag_argument.description);
    }

    return ();
}
