use std::path::PathBuf;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DevServerConfig {
    pub db_username: String,
    pub db_password: String,
    pub steamcmd_path: String,
    pub port: u16,
    pub rcon_password: String,
    pub webshare_ip: String,
    pub webshare_port: u16,
    pub map: Option<String>,
}

impl Default for DevServerConfig {
    fn default() -> Self {
        Self {
            db_username: "root".into(),
            db_password: "password".into(),
            steamcmd_path: r#"C:\path\to\steamcmd\steamcmd.exe"#.into(),
            port: 27015,
            rcon_password: rand::random::<u64>().to_string(),
            webshare_ip: "0.0.0.0".into(),
            webshare_port: rand::thread_rng().gen_range(1024..49151),
            map: Option::None,
        }
    }
}

pub fn generate_config_file(path: &PathBuf) {
    std::fs::write(
        path,
        toml::to_string_pretty(&DevServerConfig::default()).unwrap(),
    )
    .unwrap();
    println!("srcds-dev-server.toml 파일을 생성했습니다.");
}

pub fn parse_config(path: &PathBuf) -> anyhow::Result<DevServerConfig> {
    toml::from_str(&std::fs::read_to_string(&path)?).map_err(|err| anyhow::Error::new(err))
}
