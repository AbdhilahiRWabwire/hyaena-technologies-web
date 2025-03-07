use std::fmt;

// Print Help Command Output
pub fn print_help() -> () {
	println!("Hyaena Technologies Web Service");
	println!("");
	println!("");
	println!("Commands:					Description:");
	println!("");
	println!("configuration 			Configure Server with server-configuration.yaml");
	println!("exit 						Exit Server");
	println!("help						Print List of Commands and Flags");
	println!("serve 					Serve Web Applcation");
	println!("version					Print Version Number");
	println!("");
	println!("");
	println!("Flags:				    Description:");
	println!("");
	println!("--config. --c				Configure Server with server-configuration.yaml");
	println!("--exit, --e,				Exit Server");
	println!("--help, --h,				Print List of Commands and Flags");
	println!("--version, --v,			Print Version Number");

	return ();
}
