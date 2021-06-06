use rcon::Connection;

use crate::config::DevServerConfig;

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
