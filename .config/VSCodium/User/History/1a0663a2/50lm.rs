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
    std::process::Command::new("/bin/bash").arg("export").arg("LD_LIBRARY_PATH=".to_owned()+&lib_dir.display().to_string());
    println!("cargo:warning=ENVVAR: {}",env::var("LD_LIBRARY_PATH").unwrap());
}

