use std::fmt;
use std::process::exit;

// Successful Exit of the Program
pub fn succesful_exit() {
	format!("Exiting Hyaena Technologies Web Service");
	exit(0);
}

// Program Exit with Error
pub fn error_exit() {
	format!("Error: Exiting Hyaena Technologies Web Service");
	exit(1);
}
