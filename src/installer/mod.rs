
use unzip;
use serde_json;
use utils;
use helper;
use downloader;
use rpc;
use web_view::{WebView, Content, run};
use model::{Version};
use std::path::{PathBuf, Path};
use std::fs;
use std::process::Command;


pub fn open_install_helper() {
    let html = utils::generate_html(
        vec![],
        vec![include_str!("../view/js/main.js")],
    );

    let title = "Electron Platform";
    let size = (800, 480);
    let resizable = true;
    let debug = true;

    let state = vec![];

    let (tasks, _) = run(
        title,
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        rpc::init_callback,
        rpc::rpc_exec_callback,
        state
    );
}

pub fn install_runtime(v: Version) -> Result<(), String> {
    match unzip_runtime(v) {
        Err(why) => Err(why),
        Ok(unzip_path) => {
            let runtime_path = unzip_path.join("Electron.app/Contents/Frameworks");
            let target_path = helper::get_runtimes_path()
                .join(helper::version_to_string(v));
            let move_result = fs::rename(runtime_path, target_path);
            fs::remove_dir_all(&unzip_path);
            if let Ok(_) = move_result {
                Ok(())
            } else {
                Err("Move runtime files failed".to_owned())
            }
        }
    }
}

fn unzip_runtime(v: Version) -> Result<PathBuf, String> {
    let from = helper::get_platform_path()
        .join(format!("temp/{}.zip", helper::version_to_string(v)));
    let to = helper::get_platform_path()
        .join(format!("temp/{}", helper::version_to_string(v)));
    unzip_file(&from, to)
}

fn unzip_file(file_path: &PathBuf, to: PathBuf) -> Result<PathBuf, String> {
    if cfg!(target_os = "macos") {
        let result = Command::new("unzip")
            .args(&[
                "-n",
                file_path.to_str().unwrap(),
                "-d",
                to.to_str().unwrap()
            ])
            .output();
        match result {
            Ok(_) => Ok(to),
            Err(_) => Err("Failed to unzip file".to_owned())
        }
    } else {
        let file = fs::File::open(file_path).unwrap();
        let archive_result = unzip::Unzipper::new(file, &to).unzip();
        match archive_result {
            Ok(_) => Ok(to),
            Err(_) => Err("Failed to unzip file".to_owned())
        }
    }
}
