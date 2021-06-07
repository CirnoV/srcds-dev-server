// use std::os::windows::fs::symlink_dir;
// use std::path::PathBuf;

// use crate::config::DevServerConfig;

// pub(crate) fn run_miniserve(
//     root: &PathBuf,
//     config: &DevServerConfig,
// ) -> std::thread::JoinHandle<std::process::Output> {
//     let webshare = root.join("webshare");
//     let cstrike = root.join("cstrike");
//     vec!["maps", "models", "materials", "sound"]
//         .into_iter()
//         .for_each(|dir| {
//             let symlink = webshare.join(dir);
//             if symlink.exists() == false {
//                 symlink_dir(cstrike.join(dir), symlink).expect("failed to create symlink");
//             }
//         });

//     let root = root.clone();
//     let webshare_port = config.webshare_port;
//     std::thread::spawn(move || {
//         std::process::Command::new("miniserve-v0.14.0.exe")
//             .current_dir(root)
//             .arg("-i 0.0.0.0")
//             .arg("-p")
//             .arg(webshare_port.to_string())
//             .arg(webshare)
//             .output()
//             .expect("failed to execute miniserve")
//     })
// }
