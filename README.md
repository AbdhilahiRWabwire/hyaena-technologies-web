[Fleet]: https://www.jetbrains.com/fleet/
[MDN]: https://developer.mozilla.org/en-US/docs/Web/API
[Rust Language]: https://rust-lang.org
[RustRover]: https://jetbrains.com/rust
[VSCode]: https://code.visualstudio.com/docs
[WAI-ARIA]: https://www.w3.org/WAI/ARIA/apg/patterns/

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

## Build

- [Rust][Rust Language]
- [JetBrains Fleet][Fleet]
- [JetBrains RustRover][RustRover]
- [Mozilla Developer Network Web Documentation][MDN]
- [Visual Studio Code][VSCode]
- [Web Accessibility Initiative][WAI-ARIA]

## Build Web Server

```shell
git clone

cargo build
```

OR

```shell
git clone

make build
```

OR

```shell
git clone

./shell/build.sh
```

## Install Web Server

```shell
echo 'export PATH="$PATH:/usr/local/bin/htnet"' >> ~/.bashrc && sudo echo 'export PATH="$PATH:/usr/local/bin/htnet"' >> /etc/skel/.bashrc
```
