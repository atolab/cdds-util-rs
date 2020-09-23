extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-lib=ddsc");
    println!("cargo:rustc-link-lib=cdds-util");
    if !Path::new("src/cyclonedds/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "src/cyclonedds"])
            .status();
    }

    if !Path::new("src/cdds-util/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "src/cdds-util"])
            .status();
    }

    let cyclonedds_dst = Config::new("src/cyclonedds")
        .define("BUILD_IDLC", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_TESTING", "OFF")
        // .define("ENABLE_SSL", "OFF") // Disable SSL for now
        .build();

    let cdds_util_dst = Config::new("src/cdds-util")
        .define("CMAKE_C_FLAGS", format!("-I{}/include", cyclonedds_dst.display()))
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", cyclonedds_dst.display());
    println!("cargo:rustc-link-search=native={}/lib", cdds_util_dst.display());
    println!("cargo:rustc-link-lib=static=cdds-util");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_comments(false)
        .clang_arg(format!("-I{}/include", cyclonedds_dst.display()))
        .clang_arg(format!("-I{}/include", cdds_util_dst.display()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
