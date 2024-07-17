use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory.
    // (currently not used).
    // println!("cargo:rustc-link-search=/usr/lib");

    // Tell cargo to tell rustc to link the system [LIB]
    // shared library.
    // (currently not used).
    // println!("cargo:rustc-link-lib=LIB");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ffi_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/ffi");
    let ffi_src_unix = ffi_path.join("unix.c");
    let ffi_hdr_unix = ffi_path.join("unix.h");

    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed={}", ffi_src_unix.to_str().unwrap());

    // Build C source files. These will be put in the ${OUT_DIR} path.
    cc::Build::new()
        .file(ffi_src_unix.to_str().unwrap())
        .compile("unix");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(ffi_hdr_unix.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/io_bindings.rs file.
    bindings
        .write_to_file(out_path.join("io_bindings.rs"))
        .expect("Couldn't write bindings!");
}
