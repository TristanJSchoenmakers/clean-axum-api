//! <h1 align="center">clean-axum-api (🔨 UNDER CONSTRUCTION)</h1>
//!
//! <p align="center">A cleanly designed Rust REST Api</p>
//!
//! <div align="center">
//!   <!-- Github Actions -->
//!   <a href="https://github.com/TristanJSchoenmakers/clean-architecture/actions/workflows/build-validation.yml">
//!     <img src="https://img.shields.io/github/actions/workflow/status/TristanJSchoenmakers/clean-architecture/build-validation.yml?branch=main&style=flat-square"
//!       alt="actions status" />
//!   </a>
//! </div>
//!
//! <br />
//!
//!
//! ## Technologies 🔧
//!
//! - [Axum](https://github.com/tokio-rs/axum)
//! - [Sqlx](https://github.com/launchbadge/sqlx)
//! - [Xtask](https://github.com/matklad/cargo-xtask)
//!
//!
//! ## Getting started <span style="color:crimson">▶</span>
//!
//! Prerequisites for running locally:
//!
//! - [cargo](https://www.rust-lang.org/tools/install)
//! - [docker (compose)](https://docs.docker.com/engine/install/)
//!
//!
//! #### 1 - Run Docker compose
//!
//! ```bash
//! docker compose up --detach
//! ```
//!
//!
//! #### 2 - Initialize
//!
//! Run `cargo xtask --help` for a list of the other xtask commands
//!
//! ```bash
//! cargo xtask init
//! ```
//!
//!
//! #### 3 - Running the API
//!
//! ```bash
//! cargo run
//! ```

use axum::{Extension, Router};
use sqlx::PgPool;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

pub mod config;
pub mod domain;
pub mod routes;

pub fn app(db: PgPool) -> Router {
    Router::new()
        .merge(routes::router())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new())
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(Extension(db))
}
