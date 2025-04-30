FROM rust:alpine

WORKDIR /hyaena-technologies-web

COPY ./ ./

RUN cargo check \ 
cargo build --release --target x86_64-unknown-linux-gnu \ 
mv ./target/x86_64-unknown-linux-gnu/release/htnet ./binary

FROM alpine:latest

WORKDIR /hyaena-technologies-web

COPY --from=builder ./ ./ 

EXPOSE 8080/tcp

CMD ["./binary/htnet", "serve"]
