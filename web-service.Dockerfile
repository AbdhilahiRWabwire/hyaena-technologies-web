FROM rust:alpine

WORKDIR /hyaena-technologies-web

COPY ./ ./

RUN cargo check \ 
cargo build --release --target x86_64-unknown-linux-musl \ 
mv ./target/x86_64-unknown-linux-gnu/release/htnet ./binary \
rm -r ./target

FROM alpine:latest

WORKDIR /hyaena-technologies-web

COPY --from=builder ./ ./ 

EXPOSE 7878:7878/tcp

CMD ["./binary/htnet", "serve"]
