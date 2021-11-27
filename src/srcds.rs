use std::{path::PathBuf, process, thread};

use rcon::Connection;

use crate::config::DevServerConfig;

pub fn generate_server_launcher(root: &PathBuf, port: u16) {
    let launcher_path = &root.join("srcds_start.bat");
    std::fs::write(
        launcher_path,
        format!(r#"
        @echo off
        cls
        title Server Restarter
        start /high /wait srcds.exe -console -game cstrike -tickrate 66 -ip 0.0.0.0 -port {} -maxplayers 40 +map de_dust2 -autoupdate
    "#, port),
    ).unwrap();
}

pub fn run_server_launcher(root: &PathBuf) {
    let root = root.clone();
    thread::spawn(|| {
        process::Command::new("start_srcds.bat")
            .current_dir(root)
            .output()
            .expect("failed to execute srcds");

        process::exit(1);
    });
}

pub fn rcon(config: &DevServerConfig, cmd: String) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let address = format!("127.0.0.1:{}", config.port);
        let mut conn = Connection::builder()
            .connect(address, &config.rcon_password.to_string())
            .await
            .unwrap();

        conn.cmd(&cmd).await.unwrap();
    });
}
