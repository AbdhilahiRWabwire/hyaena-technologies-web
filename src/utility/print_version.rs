use std::fmt;

// Semantic Version Number Definition
struct SemanticVersionNumber {
	semantic_version: &String
}

// Print Version Number
pub fn print_version_number() -> () {
	let version_number: SemanticVersionNumber = SemanticVersionNumber {
		semantic_version: "0.2.0"
	};

	println!("Hyaena Technologies Web Service");
	println!("");
	println!("");
	println!("Version Number:		{:#?}", version_number.semantic_version);

	return ();
}
