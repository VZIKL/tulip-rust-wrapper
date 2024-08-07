extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("vendor/lib")
        .canonicalize()
        .expect("cannot canonicalize vendor path");

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=indicators");
    }
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=libindicators");
    }
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=indicatorsmacos");
    }
    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());

    let bindings = bindgen::Builder::default()
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .header("wrapper.h")
        .rustified_enum(".*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
