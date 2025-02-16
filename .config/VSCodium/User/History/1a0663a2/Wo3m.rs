use std::env;
use std::path::PathBuf;

fn main() {
    let lib_dir = PathBuf::from("./vosk");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=vosk");
}
