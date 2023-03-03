use std::{env, path::Path};
use xshell::{cmd, Shell};

fn main() {
    let emp = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let my_path = Path::new(emp).parent().unwrap();
    let sh = &Shell::new().unwrap();
    sh.change_dir(my_path);

    // 1. Run docker compose
    cmd!(sh, "docker compose up --detach").run().unwrap();

    // 2. Create database
    if cmd!(sh, "cargo sqlx --help").read().is_err() {
        cmd!(sh, "cargo install sqlx-cli").run().unwrap();
    };
    cmd!(sh, "sqlx database create").run().unwrap();

    // 3. Run migrations
    cmd!(sh, "sqlx migrate run --source api/migrations")
        .run()
        .unwrap();

    // 4. Run seeds
    // cmd!(sh, "sqlx migrate run --source api/seeds")
    //     .run()
    //     .unwrap();
}
