package utility

import (
	"fmt"
	"os"
)

// Successful Exit of the Program
func SuccesfulExit() {
	fmt.Println("Exiting Hyaena Technologies Server")
	os.Exit(0)
}

// Program Exit with Error
func ErrorExit() {
	fmt.Println("Exiting Hyaena Technologies Server")
	os.Exit(1)
}
