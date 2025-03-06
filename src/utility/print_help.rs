// Print Help Command Output
pub fn print_help() {
	Println!("Hyaena Technologies Web Service");
	Println!("");
	Println!("");
	Println!("Commands:					Description:");
	Println!("");
	Println!("configuration 			Configure Server with server-configuration.yaml");
	Println!("exit 						Exit Server");
	Println!("help						Print List of Commands and Flags");
	Println!("serve 					Serve Web Applcation");
	Println!("version					Print Version Number");
	Println!("");
	Println!("");
	Println!("Flags:				    Description:");
	Println!("");
	Println!("--config. --c				Configure Server with server-configuration.yaml");
	Println!("--exit, --e,				Exit Server");
	Println!("--help, --h,				Print List of Commands and Flags");
	Println!("--version, --v,			Print Version Number");
}
