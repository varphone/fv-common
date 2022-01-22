use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if let Ok(dir) = env::var("FV_COMMON_DIR").or(env::var("CARGO_MANIFEST_DIR")) {
        println!("cargo:rustc-env=FV_COMMON_DIR={}", dir);
    }
}
