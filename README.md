[CLIDoc]: https://github.com/HyaenaTechnologies/hyaena-technologies-web/blob/main/documentation/htnet.md
[Deno]: https://deno.land/
[Greptime Database]: https://greptime.com/
[MDN]: https://developer.mozilla.org/en-US/docs/Web/API
[Minio Database]: https://min.io/
[Postgres Database]: https://www.postgresql.org/
[Rust Language]: https://rust-lang.org
[SSH]: https://openssh.com/
[SystemD]: https://systemd.io/
[Traefik]: https://traefik.io/traefik/
[Vault]: https://www.hashicorp.com/en/products/vault
[WebGL]: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API
[WebGPU]: https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API
[W3C]: https://w3.org/TR/
[YAML]: https://yaml.org/

<a href="https://github.com/HyaenaTechnologies/hyaena-technologies-web">
  <h1>
    <picture>
      <img src="https://github.com/HyaenaTechnologies/hyaena-technologies-web/blob/main/web/assets/ht_markdown.png" alt="">
    </picture>
  </h1>
</a>

[![Rust Workflow](https://github.com/HyaenaTechnologies/hyaena-technologies-web/actions/workflows/rust.yml/badge.svg)](https://github.com/HyaenaTechnologies/hyaena-technologies-web/actions/workflows/rust.yml)

# Hyaena Technologies Web

Web Implementation of Hyaena Technologies

## Features

- Environment Variables
- [YAML Ain't Markup Language][YAML]

## Build

- [Rust][Rust Language]
- [Deno Bundler][Deno]
- [System Daemon][SystemD]
- [OpenSSH][SSH]
- [Mozilla Developer Network Web Documentation][MDN]
- [Web Consortium Documentation][W3C]
- [Web Graphics Library][WebGL] - (In Progress)
- [Web GPU][WebGPU] - (In Progress)
- [Traefik Proxy][Traefik] - (In Progress)
- [HashiCorp Vault][Vault] - (In Progress)
- [Command Line Documentation][CLIDoc]

### Databases

- **_Object Storage:_** [Minio][Minio Database] - (In Progress)

- **_Relational:_** [Postgres][Postgres Database]

- **_Time Series:_** [Greptime][Greptime Database] - (In Progress)

### Build Web Service

```shell
git clone

deno bundle ./web/src/*.ts --outdir ./web/build

cargo check

cargo test

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unkown-linux-gnu/release/htnet ./binary

./binary/htnet serve
```

### Install Web Service

```shell
sudo install ./htnet /usr/local/bin/
```
