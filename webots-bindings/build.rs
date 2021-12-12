use std::{env, path::PathBuf, process::Command};

use walkdir::WalkDir;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let status = Command::new("rsync")
        .args(["-a", "webots/", out_path.join("webots/").to_str().unwrap()])
        .status()
        .expect("Failed to execute rsync process");
    if !status.success() {
        panic!("rsync process exited with {:?}", status.code());
    }
    let status = Command::new("make")
        .args(["release"])
        .env("WEBOTS_HOME", "../../..")
        .current_dir(out_path.join("webots/src/controller/c").to_str().unwrap())
        .status()
        .expect("Failed to execute make process");
    if !status.success() {
        panic!("make process exited with {:?}", status.code());
    }

    println!(
        "cargo:rustc-link-search={}",
        out_path.join("webots/lib/controller").display()
    );
    println!("cargo:rustc-link-lib=Controller");
    println!(
        "cargo:rustc-env=LD_LIBRARY_PATH={}",
        out_path.join("webots/lib/controller").display()
    );
    println!("cargo:rerun-if-changed=wrapper.h");
    for entry in WalkDir::new("webots/include")
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| match entry.metadata().ok() {
            Some(metadata) if metadata.is_file() => Some(entry),
            _ => None,
        })
    {
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }
    for entry in WalkDir::new("webots/resources")
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| match entry.metadata().ok() {
            Some(metadata) if metadata.is_file() => Some(entry),
            _ => None,
        })
    {
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }
    for entry in WalkDir::new("webots/src")
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| match entry.metadata().ok() {
            Some(metadata) if metadata.is_file() => Some(entry),
            _ => None,
        })
        .filter(|entry| !entry.path().starts_with("webots/src/controller/c/build"))
    {
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(vec![
            "-I",
            out_path
                .join("webots/include/controller/c")
                .to_str()
                .unwrap(),
        ])
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_ZERO")
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(out_path.join("webots_bindings.rs"))
        .expect("Failed to write bindings");
}
