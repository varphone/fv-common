use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if let Ok(dir) = env::var("CARGO_MANIFEST_DIR") {
        println!("cargo:env=FV_COMMON_DIR={}", dir);
    }
}
