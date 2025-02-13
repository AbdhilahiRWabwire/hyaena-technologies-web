.PHONY: build
build:
	GOARCH=amd64 GOOS=linux go build -o ./binary/htnet ./source/main.go
