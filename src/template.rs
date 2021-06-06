use std::process::Stdio;

use crate::DevServerConfig;

pub(crate) fn run_gomplate(config: &DevServerConfig) {
    println!("Running gomplate engine...");
    std::process::Command::new("gomplate")
        .env("RCON_PASSWORD", &config.rcon_password)
        .env("SERVER_ENV", "test")
        .env("DB_USER", &config.db_username)
        .env("DB_PASSWORD", &config.db_password)
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to execute gomplate");
}
