use std::io::{StdoutLock, Write, stdout};

// Print Help Command Output
pub fn print_help_message() -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Hyaena Technologies Web Service").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Commands:					Description:").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "configuration             Configure Server with server-configuration.yaml"
    )
    .unwrap();
    writeln!(standard_output, "exit (Command Prompt)     Exit Service").unwrap();
    writeln!(
        standard_output,
        "help                      Print List of Commands and Flags"
    )
    .unwrap();
    writeln!(
        standard_output,
        "serve                     Serve Web Applcation"
    )
    .unwrap();
    writeln!(
        standard_output,
        "version                   Print Version Number"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Flags:				    Description:").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "--c                       Configure Server with server-configuration.yaml"
    )
    .unwrap();
    writeln!(
        standard_output,
        "--h                       Print List of Commands and Flags"
    )
    .unwrap();
    writeln!(
        standard_output,
        "--s                       Serve Web Applcation"
    )
    .unwrap();
    writeln!(
        standard_output,
        "--v                       Print Version Number"
    )
    .unwrap();
    writeln!(standard_output, "").unwrap();

    return ();
}
