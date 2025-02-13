FROM amd64/golang:alpine

WORKDIR /hyaena-technologies

COPY ./ ./

RUN go env \ 
go list ./source \ 
go vet ./source \ 
go fix ./source \ 
go fmt ./source \ 
GOOS=linux GOARCH=amd64 go build -o ./binary/htnet ./source/main.go

FROM amd64/alpine:latest

COPY --from=builder ./ ./

RUN ./binary/htdinet serve
