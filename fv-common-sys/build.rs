use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    cc::Build::new()
        .files(["src/dummy.c"])
        .include("include")
        .compile("fv-common");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/fv/dsp_protobuf.h")
        .header("include/fv/hi_type.h")
        .header("include/fv/log.h")
        .header("include/fv/aol_common.h")
        .header("include/fv/nn.h")
        .header("include/fv/laser_notify.h")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .anon_fields_prefix("un")
        .clang_arg("-Iinclude")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Export the DEP_FV_COMMON_DIR and DEP_BAR_INCLUDE env.
    println!("cargo:dir={}", env!("CARGO_MANIFEST_DIR"));
    println!("cargo:include={}/include", env!("CARGO_MANIFEST_DIR"));
}
