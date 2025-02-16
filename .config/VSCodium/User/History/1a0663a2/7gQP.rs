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
    env::set_var("LD_LIBRARY_PATH", lib_dir);
    println!("cargo:warning=ENVVAR: {}",env::var("LD_LIBRARY_PATH"));
}

