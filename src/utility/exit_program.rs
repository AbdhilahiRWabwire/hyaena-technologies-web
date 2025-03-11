use std::fmt;
use std::process::exit;

// Successful Exit of the Program
pub fn succesful_exit()-> ! {
	println!("Exiting Hyaena Technologies Web Service");
	
	return exit(0);
}

// Program Exit with Error
pub fn error_exit() -> ! {
	println!("Exiting Hyaena Technologies Web Service");
	
	return exit(1);
}
