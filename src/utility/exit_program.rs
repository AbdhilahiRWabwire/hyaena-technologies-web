use std::process::exit;

// Successful Exit of the Program
pub fn succesful_exit() {
	Println!("Exiting Hyaena Technologies Web Service");
	exit(0);
}

// Program Exit with Error
pub fn error_exit() {
	Println!("Exiting Hyaena Technologies Web Service");
	exit(1);
}
