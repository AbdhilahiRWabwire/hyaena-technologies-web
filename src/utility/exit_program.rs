use std::process::exit;

// Successful Exit of the Program
#[allow(dead_code)]
#[allow(unreachable_code)]
pub fn successful_exit() -> ! {
    println!("Exiting Hyaena Technologies Web Service");
    return exit(0);
}

#[allow(dead_code)]
#[allow(unreachable_code)]
// Program Exit with Error
pub fn error_exit() -> ! {
    println!("Exiting - Error(1) - Hyaena Technologies Web Service");

    return exit(1);
}
