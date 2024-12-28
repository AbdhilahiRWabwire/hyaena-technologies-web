[Dart]: https://dart.dev/
[Go Language]: https://go.dev/
[IDEA]: https://jetbrains.com/idea/
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

[![Dart Workflow](https://github.com/HyaenaTechnologies/hyaena-technologies-web/actions/workflows/dart.yml/badge.svg)](https://github.com/HyaenaTechnologies/hyaena-technologies-web/actions/workflows/dart.yml)

# Hyaena Technologies Web

Hyaena Technologies is an Open Source Technology company

## Build

- [Dart Language][Dart]
- [Go][Go Language]
- [IntelliJ IDEA][IDEA]
- [Mozilla Developer Network Web Documentation][MDN]
- [Visual Studio Code][VSCode]
- [Web Accessibility Initiative][WAI-ARIA]

```shell
git clone

dart pub upgrade

dart run build_runner build

dart run build_runner serve

dart run build_runner test

dart run build_runner watch
```

OR

```shell
git clone

dart pub upgrade

dart pub global activate webdev

echo 'export PATH="$PATH:~/.pub-cache/bin"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:~/.pub-cache/bin"' >> /etc/skel/.bashrc

webdev build

webdev serve
```

## Build Web Server

```shell
git clone

GOOS=linux GOARCH=amd64 go build -o ./binary/server ./server/source/main.go
```

## Install Web Server

```shell
echo 'export PATH="$PATH:/usr/local/bin/htnet"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/local/bin/htnet"' >> /etc/skel/.bashrc
```
