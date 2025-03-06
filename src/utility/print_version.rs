// Print Version Number

struct SemanticVersionNumber {
	semantic_version: &String
}

pub fn print_version_number() {
	let version_number: SemanticVersionNumber = SemanticVersionNumber {
		semantic_version: "0.2.0"
	};

	Println!("Hyaena Technologies Web Service");
	Println!("");
	Println!("");
	Println!("Version Number:		{:#?}", version_number.semantic_version);
}
