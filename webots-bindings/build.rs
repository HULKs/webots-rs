use std::{env, path::PathBuf, process::Command};

use walkdir::WalkDir;

fn main() {
    let status = Command::new("make")
        .args(["release"])
        .env("WEBOTS_HOME", "../../..")
        .current_dir("webots/src/controller/c")
        .status()
        .expect("Failed to execute make process");
    if !status.success() {
        panic!("make process exited with {:?}", status.code());
    }

    println!("cargo:rustc-link-search=webots/lib/controller");
    println!("cargo:rustc-link-lib=Controller");
    println!("cargo:rustc-env=LD_LIBRARY_PATH=webots/lib/controller");
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
        .clang_args(vec!["-I", "webots/include/controller/c", "-I", "webots/src/stb"])
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
