package utility

import (
	"fmt"
)

// Print Help Command Output
func PrintHelp() {
	fmt.Println("Hyaena Technologies Server")
	fmt.Println("")
	fmt.Println("")
	fmt.Println("Commands:							Description:")
	fmt.Println("")
	fmt.Println("configuration 						Configure Server with server-configuration.yaml")
	fmt.Println("exit 								Exit Server")
	fmt.Println("help								Print List of Commands and Flags")
	fmt.Println("serve 								Serve Web Applcation")
	fmt.Println("version							Print Version Number")
	fmt.Println("")
	fmt.Println("")
	fmt.Println("Flags:				    			Description:")
	fmt.Println("")
	fmt.Println("--config. -config					Configure Server with server-configuration.yaml")
	fmt.Println("--exit, -exit, --e, -e				Exit Server")
	fmt.Println("--help, -help, --h, -h				Print List of Commands and Flags")
	fmt.Println("--version, -version, --v, -v		Print Version Number")
}
