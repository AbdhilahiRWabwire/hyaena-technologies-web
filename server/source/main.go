package main

import (
	"fmt"
	"net/http"
)

const port string = ":8080"

var fileServer http.Handler = http.FileServer(http.Dir("./source"))

func main() {
	fmt.Println("Serving on Port: 8080")
	http.ListenAndServe(port, fileServer)
}
