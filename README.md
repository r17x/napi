# `@ri7nz/napi`

> 🚧 [WIP] A Nodejs utilities powered by [@napi-rs](https://github.com/napi-rs/napi-rs).

## Features

* `glob`: alternative of [glob](https://www.npmjs.com/package/glob) 

## Development

### With nix
You can skip step for [Prerequisite](#Prerequisite) when use `nix` and `direnv`, Just add `.envrc`:

```console
echo "use nix" > .envrc

direnv allow .
```

### Prerequisite

* **node** (v16.13.2)
* **rustup** (v1.24.3)
* **rustc** (v1.57.0) 

### Setup

```console
yarn install
yarn build
```

### Build
```console
yarn build

// for debug

yarn build:debug 
```
