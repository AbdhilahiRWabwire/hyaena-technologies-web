use std::{
    io::{StdoutLock, Write, stdout},
    process::ExitCode,
};

// Semantic Version Number Definition
struct SemanticVersionNumber {
    semantic_version: String,
}

// Print Version Number
pub fn print_version_number() -> ExitCode {
    let mut standard_output: StdoutLock = stdout().lock();
    let version_number: SemanticVersionNumber = SemanticVersionNumber {
        semantic_version: "0.2.0".to_string(),
    };

    writeln!(standard_output, "Hyaena Technologies Web Service").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(
        standard_output,
        "Version Number:		{}",
        version_number.semantic_version
    )
    .unwrap();

    return ExitCode::SUCCESS;
}
