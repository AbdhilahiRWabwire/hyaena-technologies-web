// Print Help Command Output
fn PrintHelp() {
	Println!("Hyaena Technologies: Data Interchange Server");
	Println!("");
	Println!("");
	Println!("Commands:							Description:");
	Println!("");
	Println!("configuration 						Configure Server with server-configuration.yaml");
	Println!("exit 								Exit Server");
	Println!("help								Print List of Commands and Flags");
	Println!("serve 								Serve Web Applcation");
	Println!("version							Print Version Number");
	Println!("");
	Println!("");
	Println!("Flags:				    			Description:");
	Println!("");
	Println!("--config. -config					Configure Server with server-configuration.yaml");
	Println!("--exit, -exit, --e, -e				Exit Server");
	Println!("--help, -help, --h, -h				Print List of Commands and Flags");
	Println!("--version, -version, --v, -v		Print Version Number");
}
