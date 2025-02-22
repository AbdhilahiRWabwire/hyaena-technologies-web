FROM rust:alpine

WORKDIR /hyaena-technologies-web

COPY ./ ./

RUN cargo check \ 
cargo build --release \ 
mv ./target/release/hyaena-technologies-web ./binary

FROM alpine:latest

WORKDIR /hyaena-technologies-web

COPY --from=builder ./ ./ 

EXPOSE 80:8080/tcp

CMD ["./binary/hyaena-technologies-web", "serve"]
