//! build.rs
//!
//! build this crate

extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

const TARGET_CPP: &str = "src/cpp/dog.cpp";
const TARGET_HPP: &str = "src/cpp/dog.hpp";

fn main() {
    println!("cargo:rerun-if-changed={}", TARGET_HPP);

    cc::Build::new()
        .file(TARGET_CPP)
        .include("src/cpp")
        .compile("dog");

    let bindings = bindgen::Builder::default()
        .header(TARGET_HPP)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("dog.rs"))
        .expect("Could not write bindings.");
}
