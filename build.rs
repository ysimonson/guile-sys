extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::str;

fn config_args(cmd: &str) -> Vec<String> {
    let out: Vec<u8> = Command::new("guile-config")
        .arg(cmd)
        .output()
        .expect(&format!("`guile-config {}` failed. Is guile installed?", cmd))
        .stdout;
    str::from_utf8(&out)
        .expect(&format!("Could not decode `guile-config {}` output as utf-8", cmd))
        .split(" ")
        .map(|s| s.to_string())
        .collect()
}

fn linker_args() -> (Vec<String>, Vec<String>) {
    let mut search_args = Vec::new();
    let mut lib_args = Vec::new();

    for arg in config_args("link") {
        if arg.starts_with("-L") {
            search_args.push(arg[2..].to_string());
        } else if arg.starts_with("-l") {
            lib_args.push(arg[2..].to_string());
        } else {
            panic!("Unknown linker arg: {}", arg);
        }
    }

    (search_args, lib_args)
}

fn main() {
    let (search_args, lib_args) = linker_args();
    let compiler_args = config_args("compile");

    for arg in search_args {
        println!("cargo:rustc-link-search={}", arg);
    }

    for arg in lib_args {
        println!("cargo:rustc-link-lib=static={}", arg);
    }

    let mut bindings = bindgen::Builder::default();

    for arg in compiler_args {
        bindings = bindings.clang_arg(arg);
    }

    let bindings = bindings.header("wrapper.h").generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(out_path).expect("Couldn't write bindings");
}
