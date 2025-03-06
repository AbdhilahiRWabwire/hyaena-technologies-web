use std::fmt;

// Print Help Command Output
pub fn print_help() {
	format!("Hyaena Technologies Web Service");
	format!("");
	format!("");
	format!("Commands:					Description:");
	format!("");
	format!("configuration 				Configure Server with server-configuration.yaml");
	format!("exit 						Exit Server");
	format!("help						Print List of Commands and Flags");
	format!("serve 						Serve Web Applcation");
	format!("version					Print Version Number");
	format!("");
	format!("");
	format!("Flags:				    	Description:");
	format!("");
	format!("--config. --c				Configure Server with server-configuration.yaml");
	format!("--exit, --e,				Exit Server");
	format!("--help, --h,				Print List of Commands and Flags");
	format!("--version, --v,			Print Version Number");
}
