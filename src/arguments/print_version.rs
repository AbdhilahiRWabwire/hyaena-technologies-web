use std::{
    io::{StdoutLock, Write, stdout},
    string::String,
};

// Semantic Version Number Definition
type SemanticVersionNumber = String;

// Print Version Number
pub fn print_version_number() -> () {
    let mut standard_output: StdoutLock = stdout().lock();
    let version_number: SemanticVersionNumber = "0.2.0".to_string();

    writeln!(standard_output, "Hyaena Technologies Web Service").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "Version Number:		{}", version_number).unwrap();

    return ();
}
