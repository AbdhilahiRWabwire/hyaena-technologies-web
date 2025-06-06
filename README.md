[Certificate Manager]: https://cert-manager.io/
[CLIDoc]: https://github.com/HyaenaTechnologies/hyaena-technologies-web/blob/main/documentation/htnet.md
[Containerman]: https://podman.io/
[Containerman Pod]: https://docs.podman.io/en/latest/markdown/podman-pod.1.html
[Dragonfly Database]: https://www.dragonflydb.io/
[Grafana]: https://grafana.com/oss/grafana/
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
[Moby Compose]: https://docs.docker.com/reference/cli/docker/compose/
[Moby Swarm]: https://docs.docker.com/reference/cli/docker/swarm/
[Rust Language]: https://rust-lang.org
[Spice Database]: https://authzed.com/
[Surreal Database]: https://surrealdb.com/
[Traefik]: https://traefik.io/traefik/
[WAI-ARIA]: https://www.w3.org/WAI/ARIA/apg/patterns/
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

- (In Progress)

## Build

- **_Container Engines:_** [Docker][Moby], [Podman][Containerman]

- **_Container Orchestration Engines:_** [Docker Compose][Moby Compose], [Docker Swarm][Moby Swarm], [KOps][K8S Ops], [Minikube][K8S Kube], [Podman Pods][Containerman Pod]

- **_Container Tools:_** [Cert-Manager][Certificate Manager], [Knative][K8S Native], [Kompose][K8S Kompose], [Kubectl][K8S Control], [Skaffold][K8S Skaffold]

- [Command Line Documentation][CLIDoc]
- [Grafana Dashboard][Grafana] - (In Progress)
- [Rust][Rust Language]
- [Mozilla Developer Network Web Documentation][MDN]
- [Traefik Proxy][Traefik] - (In Progress)
- [Web Accessibility Initiative][WAI-ARIA]

## Databases

- **_Authorization:_** [Spice][Spice Database] - (In Progress)

- **_Graph:_** [Surreal][Surreal Database]

- **_Key Value:_** [Dragonfly][Dragonfly Database] - (In Progress)

- **_Object Storage:_** [Minio][Minio Database] - (In Progress)

- **_Time Series:_** [Greptime][Greptime Database] - (In Progress)

## Build Web Service

```shell
git clone

cargo check

cargo build --release --target x86_64-unknown-linux-gnu

mv ./target/x86_64-unkown-linux-gnu/release/htnet ./binary

./binary/htnet serve
```

## Docker Build

```shell
git clone

docker build ./ --file ./web-service.Dockerfile --tag htnet:latest
||
nerdctl build ./ --file ./web-service.Dockerfile --tag htnet:latest
```

## Docker Compose

```shell
git clone

docker compose up --detach
||
nerdctl compose up --detach
```

## Kubernetes

### Kompose Kubernetes Configurations and Initialize Skaffold

```shell
git clone

kompose convert -f ./compose.yaml -o ./kubernetes

skaffold init
```

### Run Kubernetes Cluster

```shell
git clone

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
echo 'export PATH="$PATH:/usr/local/bin/htnet"' >> ~/.bashrc
```
