package utility

import (
	"fmt"
)

// Semantic Version Number
type SemanticVersionNumber struct {
	semanticVersion string
}

var versionNumber SemanticVersionNumber = SemanticVersionNumber{
	semanticVersion: "0.2.0",
}

// Print Version Number
func PrintVersionNumber() {
	fmt.Println("Hyaena Technologies Server")
	fmt.Println("")
	fmt.Println("")
	fmt.Println("Version Number:		", versionNumber.semanticVersion)
}
