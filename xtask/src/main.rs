use std::{env, path::Path};
use xshell::{cmd, Shell};

use clap::Parser;

/// cargo-xtask adds automation to a Rust project, see: https://github.com/matklad/cargo-xtask
#[derive(Parser)]
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
enum Cargo {
    /// Initializes the project by running the required docker containers & migrating the database
    Init,
    /// Does all the checks that the build/validation pipeline does
    Check,
}

fn main() {
    let parser = Cargo::parse();
    match parser {
        Cargo::Init => init(),
        Cargo::Check => check(),
    }
}

fn init() {
    let emp = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let my_path = Path::new(emp).parent().unwrap();
    let sh = &Shell::new().unwrap();
    sh.change_dir(my_path);

    // 1. Run docker compose
    if cmd!(sh, "docker compose --help").read().is_err() {
        eprintln!("Cannot find docker compose, is docker compose not installed?");
        std::process::exit(-1);
    };
    cmd!(sh, "docker compose up --detach").run().unwrap();

    // 2. Setup database
    if cmd!(sh, "cargo sqlx --help").read().is_err() {
        cmd!(sh, "cargo install sqlx-cli").run().unwrap();
    };
    cmd!(sh, "sqlx database setup").run().unwrap();
}

fn check() {
    let emp = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let my_path = Path::new(emp).parent().unwrap();
    let sh = &Shell::new().unwrap();
    sh.change_dir(my_path);

    // 1. Check for clippy analyzer warnings/errors
    cmd!(
        sh,
        "cargo clippy --all-targets --all-features -- -D warnings"
    )
    .run()
    .unwrap();

    // 2. Check if project is formatted correctly
    cmd!(sh, "cargo fmt --check").run().unwrap();

    // 3. check crate dependencies for security vulnerabilities
    if cmd!(sh, "cargo audit --help").read().is_err() {
        cmd!(sh, "cargo install cargo-audit").run().unwrap();
    };
    cmd!(sh, "cargo audit").run().unwrap();
}
