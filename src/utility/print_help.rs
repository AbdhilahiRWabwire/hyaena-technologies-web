// Print Help Command Output
pub fn print_help_message() -> () {
    println!("Hyaena Technologies Web Service");
    println!("");
    println!("");
    println!("Commands:					Description:");
    println!("");
    println!("configuration             Configure Server with server-configuration.yaml");
    println!("exit                      Exit Service");
    println!("help                      Print List of Commands and Flags");
    println!("serve                     Serve Web Applcation");
    println!("version                   Print Version Number");
    println!("");
    println!("Flags:				    Description:");
    println!("--configuration --c       Configure Server with server-configuration.yaml");
    println!("--help --h                Print List of Commands and Flags");
    println!("--serve --s               Serve Web Applcation");
    println!("--version --v             Print Version Number");
    println!("");

    return ();
}