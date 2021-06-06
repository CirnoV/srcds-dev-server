use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct DevServerConfig {
    pub db_username: String,
    pub db_password: String,
    pub steamcmd_path: String,
    pub port: u16,
    pub rcon_password: String,
}

pub fn generate_config_file(path: &PathBuf) {
    std::fs::write(
        path,
        toml::to_string_pretty(&DevServerConfig::default()).unwrap(),
    )
    .unwrap();
    println!("touhou-dev-server.toml 파일을 생성했습니다.");
}

pub fn parse_config(path: &PathBuf) -> anyhow::Result<DevServerConfig> {
    toml::from_str(&std::fs::read_to_string(&path)?).map_err(|err| anyhow::Error::new(err))
}
