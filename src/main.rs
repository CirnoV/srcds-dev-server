pub(crate) mod config;
// pub(crate) mod fastdl;
pub(crate) mod srcds;
pub(crate) mod steamcmd;
pub(crate) mod template;

use std::{
    env,
    path::{Path, PathBuf},
};

use anyhow::Result;
use srcds::{generate_server_launcher, run_server_launcher};
use structopt::StructOpt;
use template::generate_gomplate_template;
use watchexec::{
    config::{Config, ConfigBuilder},
    error::Error,
    pathop::PathOp,
    run::{watch, ExecHandler, Handler},
};

use crate::{
    config::{generate_config_file, parse_config, DevServerConfig},
    // fastdl::run_miniserve,
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
    env::current_dir().unwrap().parent().unwrap().to_path_buf()
}

fn main() -> Result<()> {
    let root = get_root_dir();
    let config_path = root.join("srcds-dev-server.toml");
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
            // start_srcds.bat 생성
            generate_server_launcher(&root, server_config.port);
            // .gomplate.yaml 생성
            generate_gomplate_template(&root);
            // gomplate 엔진 실행
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

            // start_srcds.bat 실행
            run_server_launcher(&root);

            // let _handle = run_miniserve(&root, &server_config);

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
        srcds::rcon(&self.config, "sm_restartmap".into());
        self.handler.on_update(ops)
    }
}
