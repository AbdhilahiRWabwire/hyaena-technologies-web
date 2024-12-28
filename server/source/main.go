package main

import (
	"fmt"
	"net/http"
)

func main() {
	const port string = ":8080"
	var fileServer http.Handler = http.FileServer(http.Dir("./lib"))
	fmt.Println("Serving on Port: 8080")
	http.ListenAndServe(port, fileServer)
}
