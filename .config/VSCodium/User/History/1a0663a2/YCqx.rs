extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let lib_dir = PathBuf::from("/home/arch/Projects/Rust/voice/vosk");

    // Print the path to verify it
    println!("cargo:warning=Linking Vosk from: {}", lib_dir.display());

    // Link search path and library
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=vosk");

    // Optional: Set environment variables to ensure the library is found at runtime
    let bindings = bindgen::Builder::default()
        .header("/usr/include/espeak-ng/speak_lib.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:warning=ENVVAR: {}",env::var("LD_LIBRARY_PATH").unwrap());
}

