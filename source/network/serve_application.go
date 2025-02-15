package network

import (
	"fmt"
	"net/http"
)

const port string = ":8080"

var fileServer http.Handler = http.FileServer(http.Dir("./web/source"))

var router *http.ServeMux = http.NewServeMux()

var routes [2]string = [2]string{
	"/",
	"/about",
}

// Serve Web Application
func ServeWebApplication() {
	router.Handle(routes[1], fileServer)
	fmt.Println("Serving on Port: 8080")
	http.ListenAndServe(port, router)
}
