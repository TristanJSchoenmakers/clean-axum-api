<h1 align="center">clean-architecture (🔨 UNDER CONSTRUCTION)</h1>

<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/TristanJSchoenmakers/clean-architecture/actions/workflows/build-validation.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/TristanJSchoenmakers/clean-architecture/build-validation.yml?branch=main&style=flat-square"
      alt="actions status" />
  </a>
</div>

<br />

A cleanly designed Rust REST Api


## Technologies

- [Axum](https://github.com/tokio-rs/axum)
- [Sqlx](https://github.com/launchbadge/sqlx)
- [Xtask](https://github.com/matklad/cargo-xtask)


## Getting started

Prerequisites:

- [cargo](https://www.rust-lang.org/tools/install)
- [docker (compose)](https://docs.docker.com/engine/install/)


#### Initialize

Run `cargo xtask --help` for a list of the other xtask commands

``` bash
cargo xtask init
```rust


#### Running the API

``` bash
cargo run
```
