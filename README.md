[Fleet]: https://www.jetbrains.com/fleet/
[GoLand]: https://jetbrains.com/go
[Go Language]: https://go.dev/
[MDN]: https://developer.mozilla.org/en-US/docs/Web/API
[VSCode]: https://code.visualstudio.com/docs
[WAI-ARIA]: https://www.w3.org/WAI/ARIA/apg/patterns/

<a href="https://github.com/HyaenaTechnologies/hyaena-technologies-web">
  <h1>
    <picture>
      <img src="https://github.com/HyaenaTechnologies/hyaena-technologies-web/blob/main/assets/ht_markdown.png" alt="">
    </picture>
  </h1>
</a>

[![Go Workflow](https://github.com/HyaenaTechnologies/hyaena-technologies-web/actions/workflows/go.yml/badge.svg)](https://github.com/HyaenaTechnologies/hyaena-technologies-web/actions/workflows/go.yml)

# Hyaena Technologies Web

Hyaena Technologies is an Open Source Technology company

## Build

- [Go][Go Language]
- [JetBrains Fleet][Fleet]
- [JetBrains GoLand][GoLand]
- [Mozilla Developer Network Web Documentation][MDN]
- [Visual Studio Code][VSCode]
- [Web Accessibility Initiative][WAI-ARIA]

## Build Web Server

```shell
git clone

GOOS=linux GOARCH=amd64 go build -o ./binary/htnet ./source/main.go
```

## Install Web Server

```shell
echo 'export PATH="$PATH:/usr/local/bin/htnet"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/local/bin/htnet"' >> /etc/skel/.bashrc
```
