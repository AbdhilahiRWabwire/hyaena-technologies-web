[CLIDoc]: https://github.com/HyaenaTechnologies/hyaena-technologies-web/blob/main/documentation/htnet.md
[Containerman]: https://podman.io/
[Containerman Pod]: https://docs.podman.io/en/latest/markdown/podman-pod.1.html
[Etcd-IO]: https://etcd.io/
[Fleet]: https://www.jetbrains.com/fleet/
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
[RustRover]: https://jetbrains.com/rust
[Spice Database]: https://authzed.com/
[Surreal Database]: https://surrealdb.com/
[VSCode]: https://code.visualstudio.com/docs
[WAI-ARIA]: https://www.w3.org/WAI/ARIA/apg/patterns/
[YAML]: https://yaml.org/

<a href="https://github.com/HyaenaTechnologies/hyaena-technologies-web">
  <h1>
    <picture>
      <img src="https://github.com/HyaenaTechnologies/hyaena-technologies-web/blob/main/assets/ht_markdown.png" alt="">
    </picture>
  </h1>
</a>

[![Rust Workflow](https://github.com/HyaenaTechnologies/hyaena-technologies-web/actions/workflows/rust.yml/badge.svg)](https://github.com/HyaenaTechnologies/hyaena-technologies-web/actions/workflows/rust.yml)

# Hyaena Technologies Web

Hyaena Technologies is an Open Source Technology company

## Features

- Environment Variables

## Build

- **_Container Engines:_** [Docker][Moby], [Podman][Containerman]

- **_Container Orchestration Engines:_** [Docker Compose][Moby Compose], [Docker Swarm][Moby Swarm], [KOps][K8S Ops], [Minikube][K8S Kube], [Podman Pods][Containerman Pod]

- **_Container Tools:_** [Knative][K8S Native], [Kompose][K8S Kompose], [Kubectl][K8S Control], [Skaffold][K8S Skaffold]

- [Command Line Documentation][CLIDoc]
- [Rust][Rust Language]
- [JetBrains Fleet][Fleet]
- [JetBrains RustRover][RustRover]
- [Mozilla Developer Network Web Documentation][MDN]
- [Visual Studio Code][VSCode]
- [Web Accessibility Initiative][WAI-ARIA]

## Databases

- **_Authorization:_** [Spice][Spice Database] - (In Progress)

- **_Graph:_** [Surreal][Surreal Database]

- **_Object Storage:_** [Minio][Minio Database] - (In Progress)

- **_Time Series:_** [Greptime][Greptime Database] - (In Progress)

## Build Web Server

```shell
git clone

cargo build --release --target x86_64-unknown-linux-gnu
mv ./target/x86_64-unkown-linux-gnu/release/htnet ./binary
./binary/htnet serve
```

## Containerize Web Server

```shell
git clone

docker build ./ --file ./hyaena-technologies.Dockerfile --tag hyaena-technologies-web:latest
```

## Install Web Server

```shell
echo 'export PATH="$PATH:/usr/local/bin/htnet"' >> ~/.bashrc
```
