# Pathfinder

[Try it](https://thomas.sauvajon.tech/pathfinder)

- Rust
- Yew (Web Assembly front-end framework)
- Dijsktra's algorithm (kind of)

Theme: [Dracula](https://github.com/dracula/dracula-theme)

## Getting started

### Install Rust
See instructions at https://rustup.rs

### Get the dependencies
```sh
$ cargo build
```

### Build the app
```
$ make build
```
This will create JavaScript/WebAssembly files in `./docs`. Why `./docs` and not `./static`? Because this project is hosted with GitHub pages which serves `./docs`.

You can use any web server to serve the `./docs` folder. To use `miniserve` (a simple Rust web server), run `make dependencies && make serve` and browse http://127.0.0.1:8888.
