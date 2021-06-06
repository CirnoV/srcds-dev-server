use std::{path::PathBuf, process::Stdio};

pub fn steamcmd_update_app(steamcmd: &PathBuf, server_path: &PathBuf) {
    std::process::Command::new(steamcmd)
        .arg("+login anonymous")
        .arg("+force_install_dir")
        .arg(server_path.as_os_str())
        .arg("+app_update 232330")
        .arg("+quit")
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to execute steamcmd");
}
