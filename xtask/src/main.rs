use std::{env, path::Path};
use xshell::{cmd, Shell};

fn main() {
    let emp = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let my_path = Path::new(emp).parent().unwrap();
    let sh = &Shell::new().unwrap();
    sh.change_dir(my_path);

    // 1. Run docker compose
    cmd!(sh, "docker compose up --detach").run().unwrap();

    // 2. Setup database
    if cmd!(sh, "cargo sqlx --help").read().is_err() {
        cmd!(sh, "cargo install sqlx-cli").run().unwrap();
    };
    cmd!(sh, "sqlx database setup").run().unwrap();
}
