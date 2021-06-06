pub(crate) mod config;
pub(crate) mod srcds;
pub(crate) mod steamcmd;
pub(crate) mod template;

use std::{
    env,
    path::{Path, PathBuf},
    thread,
};

use anyhow::Result;
use structopt::StructOpt;
use watchexec::{
    config::{Config, ConfigBuilder},
    error::Error,
    pathop::PathOp,
    run::{watch, ExecHandler, Handler},
};

use crate::{
    config::{generate_config_file, parse_config, DevServerConfig},
    steamcmd::steamcmd_update_app,
    template::run_gomplate,
};

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Install,
    Run,
}

fn get_root_dir() -> PathBuf {
    env::current_exe().unwrap().parent().unwrap().to_path_buf()
}

fn generate_server_launcher(root: &PathBuf, port: u16) {
    std::fs::write(
        &root.join("서버실행.bat"),
        format!(r#"
        @echo off
        cls
        title Server Restarter
        start /high /wait srcds.exe -console -game cstrike -tickrate 66 -ip 0.0.0.0 -port {} -maxplayers 40 +map zm_cyland2_sg_fix -autoupdate
    "#, port),
    ).unwrap();
}

fn main() -> Result<()> {
    let root = get_root_dir();
    let config_path = root.join("touhou-dev-server.toml");
    let server_config: DevServerConfig = match parse_config(&config_path) {
        Ok(config) => config,
        Err(_) => {
            generate_config_file(&config_path);
            return Ok(());
        }
    };

    let opt = Opt::from_args();
    match opt.cmd {
        Command::Install => {
            steamcmd_update_app(
                &Path::new(&server_config.steamcmd_path).to_path_buf(),
                &root,
            );
            return Ok(());
        }
        Command::Run => {
            generate_server_launcher(&root, server_config.port);
            run_gomplate(&root, &server_config);

            let cstrike = &root.join("cstrike");
            let sourcemod = &cstrike.join("addons").join("sourcemod");
            let plugins = &sourcemod.join("plugins");

            let config = ConfigBuilder::default()
                .clear_screen(true)
                .run_initially(true)
                .no_vcs_ignore(false)
                .no_ignore(false)
                .paths(vec![plugins.into()])
                .filters(vec!["*.smx".into()])
                .cmd(vec!["".into()])
                .build()?;

            {
                let root = root.clone();
                thread::spawn(|| {
                    std::process::Command::new("서버실행.bat")
                        .current_dir(root)
                        .output()
                        .expect("failed to execute srcds");

                    std::process::exit(1);
                });
            }

            let handler = MyHandler {
                root: root,
                config: server_config.clone(),
                handler: ExecHandler::new(config)?,
            };
            watch(&handler).map_err(|err| anyhow::Error::new(err))
        }
    }
}

struct MyHandler {
    root: PathBuf,
    config: DevServerConfig,
    handler: ExecHandler,
}

impl Handler for MyHandler {
    fn args(&self) -> Config {
        self.handler.args()
    }

    fn on_manual(&self) -> Result<bool, Error> {
        run_gomplate(&self.root, &self.config);
        self.handler.on_manual()
    }

    fn on_update(&self, ops: &[PathOp]) -> Result<bool, Error> {
        run_gomplate(&self.root, &self.config);
        srcds::rcon(&self.config, "sm_map esseland".into());
        self.handler.on_update(ops)
    }
}
