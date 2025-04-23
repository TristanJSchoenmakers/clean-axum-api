use clap::Parser;
use std::{env, path::Path};
use xshell::{cmd, Shell};

/// cargo-xtask adds automation to a Rust project, see: https://github.com/matklad/cargo-xtask
#[derive(Parser)]
#[clap(name = "cargo xtask")]
#[clap(bin_name = "cargo xtask")]
enum Cargo {
    /// Initializes the project by running the required docker containers & migrating the database
    Init,
    /// Does all the checks that the build/validation pipeline does
    Check,
    /// Syncs lib.rs documentation with README.md
    DocGen,
}

fn main() {
    let emp = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let my_path = Path::new(emp).parent().unwrap();
    let sh = &Shell::new().unwrap();
    sh.change_dir(my_path);

    match Cargo::parse() {
        Cargo::Init => init(sh),
        Cargo::Check => check(sh),
        Cargo::DocGen => doc_gen(sh),
    }
}

fn init(sh: &Shell) {
    // 1. Setup database
    cmd!(sh, "cargo install sqlx-cli").run().unwrap();
    cmd!(sh, "sqlx database setup").run().unwrap();
    // 2. Install Drill for load testing: https://github.com/fcsonline/drill
    cmd!(sh, "cargo install drill").run().unwrap();
    // 3. Install cargo-readme for syncing lib.rs with Readme.md: https://github.com/webern/cargo-readme
    cmd!(sh, "cargo install cargo-readme").run().unwrap();
    // 4. Install cargo-nextest a better test runner: https://github.com/nextest-rs/nextest
    cmd!(sh, "cargo install  ycargo-nextest").run().unwrap();
}

fn check(sh: &Shell) {
    // 1. Check for clippy analyzer warnings/errors
    cmd!(
        sh,
        "cargo clippy --all-targets --all-features -- -D warnings -D clippy::unwrap_used"
    )
    .run()
    .unwrap();

    // 2. Check if project is formatted correctly
    cmd!(sh, "cargo fmt --all -- --check").run().unwrap();
}

fn doc_gen(sh: &Shell) {
    // 1. Sync lib.rs documentation to README.md
    if cmd!(sh, "cargo readme --help")
        .ignore_stderr()
        .ignore_stdout()
        .quiet()
        .run()
        .is_err()
    {
        cmd!(sh, "cargo install cargo-readme").run().unwrap();
    };

    let dir = sh.current_dir();
    let readme_path = dir.to_str().unwrap();

    cmd!(
        sh,
        "cargo readme -o {readme_path}/README.md --no-title --no-indent-headings"
    )
    .run()
    .unwrap();
}
