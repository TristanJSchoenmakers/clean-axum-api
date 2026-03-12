<h1 align="center">clean-axum-api (🔨 UNDER CONSTRUCTION)</h1>

<p align="center">A cleanly designed Rust REST Api</p>

<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/TristanJSchoenmakers/clean-architecture/actions/workflows/build-validation.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/TristanJSchoenmakers/clean-architecture/build-validation.yml?branch=main&style=flat-square"
      alt="actions status" />
  </a>
</div>

<br />


## Technologies 🔧

- [Axum](https://github.com/tokio-rs/axum)
- [Sqlx](https://github.com/launchbadge/sqlx)
- [Drill](https://github.com/fcsonline/drill)


## Getting started <span style="color:crimson">▶</span>

Prerequisites for running locally:

- [Nix](https://nixos.org/download/)
- [docker (compose)](https://docs.docker.com/engine/install/)


#### 1 - Enter the development shell

This project uses a Nix development shell to provide the required tooling.

```bash
nix develop
```

#### 2 - Start the database

```bash
docker compose up --detach
```


#### 3 - Setup the database

```bash
sqlx database setup
```


#### 4 - Running the API

```bash
cargo run
```
