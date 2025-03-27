use std::process::ExitCode;

// Print Help Command Output
pub fn print_help_message() -> ExitCode {
    println!("Hyaena Technologies Web Service");
    println!("");
    println!("");
    println!("Commands:					Description:");
    println!("");
    println!("configuration             Configure Server with server-configuration.yaml");
    println!("exit (Command Prompt)     Exit Service");
    println!("help                      Print List of Commands and Flags");
    println!("serve                     Serve Web Applcation");
    println!("version                   Print Version Number");
    println!("");
    println!("Flags:				    Description:");
    println!("--c                       Configure Server with server-configuration.yaml");
    println!("--h                       Print List of Commands and Flags");
    println!("--s                       Serve Web Applcation");
    println!("--v                       Print Version Number");
    println!("");

    return ExitCode::SUCCESS;
}
