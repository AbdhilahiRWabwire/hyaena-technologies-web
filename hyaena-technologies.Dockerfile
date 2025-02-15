FROM amd64/rust:alpine

WORKDIR /hyaena-technologies

COPY ./ ./

RUN cargo build

FROM amd64/alpine:latest

WORKDIR /hyaena-technologies

COPY --from=builder ./ ./ 

RUN ./target/debug/htnet serve
