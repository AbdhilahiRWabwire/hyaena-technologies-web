[Go Language]: https://go.dev/
[JavaScript Language]: https://developer.mozilla.org/en-US/docs/Web/JavaScript
[MDN]: https://developer.mozilla.org/en-US/docs/Web/API
[VSCode]: https://code.visualstudio.com/docs
[WAI-ARIA]: https://www.w3.org/WAI/ARIA/apg/patterns/
[Webstorm]: https://jetbrains.com/webstorm

<a href="https://github.com/HyaenaTechnologies/hyaena-technologies-web">
  <h1>
    <picture>
      <img src="https://github.com/HyaenaTechnologies/hyaena-technologies-web/blob/main/assets/ht_markdown.png" alt="">
    </picture>
  </h1>
</a>

[![Dart Workflow](https://github.com/HyaenaTechnologies/hyaena-technologies-web/actions/workflows/dart.yml/badge.svg)](https://github.com/HyaenaTechnologies/hyaena-technologies-web/actions/workflows/dart.yml)

# Hyaena Technologies Web

Hyaena Technologies is an Open Source Technology company

## Build

- [Go][Go Language]
- [JetBrains Webstorm][Webstorm]
- [JavaScript][JavaScript Language]
- [Mozilla Developer Network Web Documentation][MDN]
- [Visual Studio Code][VSCode]
- [Web Accessibility Initiative][WAI-ARIA]

## Build Web Server

```shell
git clone

GOOS=linux GOARCH=amd64 go build -o ./binary/htnet ./server/source/main.go
```

## Install Web Server

```shell
echo 'export PATH="$PATH:/usr/local/bin/htnet"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/local/bin/htnet"' >> /etc/skel/.bashrc
```
