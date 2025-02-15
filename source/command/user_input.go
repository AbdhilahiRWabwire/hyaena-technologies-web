package command

import (
	"fmt"
	"hyaena-technologies-server/source/utility"
	"os"
)

// Command Line User Input
func UserInput() {
	var inputArguments []string = os.Args[1:]

	var tokenizedInputArguments []string = TokenizeArgumentInput(inputArguments[0])

	var availableCommands map[string]InputCommand = CommandMap()

	var inputCommand InputCommand = availableCommands[tokenizedInputArguments[0]]

	if len(os.Args) < 1 || len(inputArguments) == 0 {
		fmt.Println("Input Required")
		utility.ErrorExit()
	} else {
		inputCommand.event()
	}
}
