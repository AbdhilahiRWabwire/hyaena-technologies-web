FROM rust:alpine

WORKDIR /hyaena-technologies

COPY ./ ./

RUN cargo check \  
cargo build --release \ 
mv ./target/release/hyaena-technologies-server ./binary

FROM alpine:latest

WORKDIR /hyaena-technologies

COPY --from=builder ./ ./ 

EXPOSE 80:8080/tcp

RUN ./binary/hyaena-technologies-server serve
