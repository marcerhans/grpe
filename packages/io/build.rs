use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-linkn-search=/usr/lib");
    println!("cargo:rustc-link-lib=X11");

    let bindings = bindgen::Builder::default()
        .header("/usr/include/X11/Xlib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("unix_x11_bindings.rs"))
        .expect("Couldn't write bindings!");
}
