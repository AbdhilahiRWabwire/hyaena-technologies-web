package command

import (
	"strings"
)

// Tokenize Command Line Arguments from User Input
func TokenizeArgumentInput(input string) []string {
	var toLowerCase string = strings.ToLower(input)

	var inputFields []string = strings.Fields(toLowerCase)

	return inputFields
}
