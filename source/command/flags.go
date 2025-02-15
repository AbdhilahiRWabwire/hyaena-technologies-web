package command

import (
	"hyaena-technologies-server/source/utility"
)

// User Input Flag
type InputFlag struct {
	name        string
	description string
	event       func()
}

// Map of Input Flags
func FlagMap() map[string]InputFlag {
	return map[string]InputFlag{
		"--exit": {
			name:        "exit",
			description: "Exit Server",
			event:       utility.SuccesfulExit,
		},
		"-exit": {
			name:        "exit",
			description: "Exit Server",
			event:       utility.SuccesfulExit,
		},
		"--e": {
			name:        "exit",
			description: "Exit Server",
			event:       utility.SuccesfulExit,
		},
		"-e": {
			name:        "exit",
			description: "Exit Server",
			event:       utility.SuccesfulExit,
		},
		"--help": {
			name:        "help",
			description: "Print List of Commands and Flags",
			event:       utility.PrintHelp,
		},
		"-help": {
			name:        "help",
			description: "Print List of Commands and Flags",
			event:       utility.PrintHelp,
		},
		"--h": {
			name:        "help",
			description: "Print List of Commands and Flags",
			event:       utility.PrintHelp,
		},
		"-h": {
			name:        "help",
			description: "Print List of Commands and Flags",
			event:       utility.PrintHelp,
		},
		"--version": {
			name:        "version",
			description: "Print Version Number",
			event:       utility.PrintVersionNumber,
		},
		"-version": {
			name:        "version",
			description: "Print Version Number",
			event:       utility.PrintVersionNumber,
		},
		"--v": {
			name:        "version",
			description: "Print Version Number",
			event:       utility.PrintVersionNumber,
		},
		"-v": {
			name:        "version",
			description: "Print Version Number",
			event:       utility.PrintVersionNumber,
		},
	}
}
