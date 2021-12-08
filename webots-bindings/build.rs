use std::{env, fs::create_dir_all, path::PathBuf, process::Command};

use tempfile::Builder;

fn download(url: &String, downloaded_file_path: &PathBuf) {
    let status = Command::new("wget")
        .args([
            "--no-verbose",
            "--output-document",
            downloaded_file_path
                .as_os_str()
                .to_str()
                .expect("Failed to convert to str"),
            url.as_str(),
        ])
        .status()
        .expect("Failed to execute wget process");
    if !status.success() {
        panic!("wget process exited with {:?}", status.code());
    }
}

fn extract(downloaded_file_path: &PathBuf, target_directory: &PathBuf) {
    create_dir_all(target_directory).expect("Failed to create target directory");
    let status = Command::new("tar")
        .args([
            "--extract",
            "--bzip2",
            "--file",
            downloaded_file_path
                .as_os_str()
                .to_str()
                .expect("Failed to convert to str"),
        ])
        .current_dir(
            target_directory
                .as_os_str()
                .to_str()
                .expect("Failed to convert to str"),
        )
        .status()
        .expect("Failed to execute tar process");
    if !status.success() {
        panic!("tar process exited with {:?}", status.code());
    }
}

fn get_webots_path() -> PathBuf {
    if let Some(path) = env::var("WEBOTS_PATH").ok() {
        let webots_path = PathBuf::from(&path);
        if !webots_path.exists() {
            panic!("Passed WEBOTS_PATH {:?} does not exist.", webots_path);
        }
        webots_path
    } else if cfg!(target_os = "linux") {
        let webots_path = PathBuf::from("/usr/local/webots");
        if !webots_path.exists() {
            println!("cargo:warning=Guessed Webots directory {:?} does not exist and WEBOTS_PATH not set. Downloading...", webots_path);
            let temporary_directory = Builder::new()
                .prefix("webots-bindings-")
                .tempdir()
                .expect("Failed to create temporary directory");
            let temporary_directory_path = temporary_directory.into_path();
            let downloaded_file_path = temporary_directory_path.join("latest.tar.bz2");
            download(&"https://github.com/cyberbotics/webots/releases/latest/download/webots-R2021b-x86-64.tar.bz2".to_string(), &downloaded_file_path);
            let target_directory = temporary_directory_path.join("latest");
            extract(&downloaded_file_path, &target_directory);
            return target_directory.join("webots");
        }
        webots_path
    } else if cfg!(target_os = "macos") {
        let webots_path = PathBuf::from("/Applications/Webots.app");
        if !webots_path.exists() {
            panic!(
                "Guessed Webots directory {:?} does not exist and WEBOTS_PATH not set.",
                webots_path
            );
        }
        webots_path
    } else if cfg!(target_os = "windows") {
        let webots_path = PathBuf::from("C:\\Program Files\\Webots");
        if !webots_path.exists() {
            panic!(
                "Guessed Webots directory {:?} does not exist and WEBOTS_PATH not set.",
                webots_path
            );
        }
        webots_path
    } else {
        panic!("Failed to detect OS and WEBOTS_PATH not set.");
    }
}

fn main() {
    let webots_path = get_webots_path();
    let lib_path = PathBuf::from(&webots_path).join("lib/controller");
    let include_path = PathBuf::from(&webots_path).join("include/controller/c");

    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=Controller");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", lib_path.display());
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(vec!["-I", include_path.to_str().unwrap()])
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_ZERO")
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("webots_bindings.rs"))
        .expect("Failed to write bindings");
}
