[CLIDoc]: https://github.com/HyaenaTechnologies/hyaena-technologies-web/blob/main/documentation/htnet.md
[Containerman]: https://podman.io/
[Dragonfly Database]: https://www.dragonflydb.io/
[Greptime Database]: https://greptime.com/
[K8S Control]: https://kubernetes.io/
[K8S Kompose]: https://kompose.io/
[K8S Kube]: https://minikube.sigs.k8s.io/docs/
[K8S Native]: https://knative.dev/docs/
[K8S Ops]: https://kops.sigs.k8s.io/
[K8S Skaffold]: https://skaffold.dev/
[MDN]: https://developer.mozilla.org/en-US/docs/Web/API
[Minio Database]: https://min.io/
[Moby]: http://docker.com
[Postgres Database]: https://www.postgresql.org/
[Rust Language]: https://rust-lang.org
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

Hyaena Technologies is an Open Source Technology company

## Features

- Environment Variable Configuration
- [YAML Configuration][YAML]

## Build

- **_Containerization Tools:_** [Docker][Moby], [Knative][K8S Native], [Kompose][K8S Kompose], [KOps][K8S Ops], [Kubectl][K8S Control], [Minikube][K8S Kube], [Podman][Containerman], [Skaffold][K8S Skaffold]

- **_Secret Management:_** [HashiCorp Vault][Vault] - (In Progress)

- [Rust][Rust Language]
- [Traefik Proxy][Traefik] - (In Progress)
- [Command Line Documentation][CLIDoc]
- [Mozilla Developer Network Web Documentation][MDN]
- [Web Consortium Documentation][W3C]
- [Web Graphics Library][WebGL] - (In Progress)
- [Web GPU][WebGPU] - (In Progress)

## Databases

- **_Key Value:_** [Dragonfly][Dragonfly Database] - (In Progress)

- **_Object Storage:_** [Minio][Minio Database] - (In Progress)

- **_Relational:_** [Postgres][Postgres Database]

- **_Time Series:_** [Greptime][Greptime Database] - (In Progress)

## Build Web Service

```shell
git clone

cargo check

cargo test

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unkown-linux-gnu/release/htnet ./binary

./binary/htnet serve
```

## Docker Build

```shell
docker build ./ --file ./web-service.Dockerfile --tag htnet:0.2.0

||

nerdctl build ./ --file ./web-service.Dockerfile --tag htnet:0.2.0
```

## Docker Compose

```shell
docker compose up --detach

||

nerdctl compose up --detach
```

## Kubernetes

### Kompose Kubernetes Configurations and Initialize Skaffold

```shell
kompose convert -f ./compose.yaml -o ./kubernetes

skaffold init
```

### Run Kubernetes Cluster

```shell
minikube kubectl apply -f './certificate-manager/*.yaml'

minikube kubectl apply -f './knative/eventing/*.yaml'

minikube kubectl apply -f './knative/serving/*.yaml'
 
minikube kubectl apply -f './kubernetes/*.yaml'

||

skaffold dev # Development Mode

skaffold run # Production Mode
```

## Install Web Service

```shell
sudo install ./htnet /usr/local/bin/
```
