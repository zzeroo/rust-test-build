extern crate bindgen;
extern crate make_cmd;
extern crate pkg_config;

use make_cmd::make;
use std::env;
// use std::fs::{self, File};
// use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

const LIBMODBUS_DIR: &'static str = "libmodbus";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // If pkg-config is present and detects libmodbus, return and do nothing
    // let has_pkgconfig = Command::new("pkg-config").output().is_ok();

    if let Ok(library) = pkg_config::find_library("libmodbus") {
        // println!("{:?}", library);
        run_bindgen()
    }
}

fn run_bindgen() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        .no_unstable_rust()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-I/usr/include/modbus")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");


    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(".");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
