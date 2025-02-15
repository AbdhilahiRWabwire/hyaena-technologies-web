package command

import (
	"hyaena-technologies-server/source/network"
	"hyaena-technologies-server/source/utility"
)

// User Input Command
type InputCommand struct {
	name        string
	description string
	event       func()
}

// Map of Input Commands
func CommandMap() map[string]InputCommand {
	return map[string]InputCommand{
		"exit": {
			name:        "exit",
			description: "Exit Server",
			event:       utility.SuccesfulExit,
		},
		"help": {
			name:        "help",
			description: "Print List of Commands and Flags",
			event:       utility.PrintHelp,
		},
		"serve": {
			name:        "serve",
			description: "Serve Web Applcation",
			event:       network.ServeWebApplication,
		},
		"version": {
			name:        "version",
			description: "Print Version Number",
			event:       utility.PrintVersionNumber,
		},
	}
}
