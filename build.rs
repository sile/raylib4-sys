use std::env;
use std::path::PathBuf;

fn main() {
    // raylib's `make install` puts the library under /usr/local/lib, which
    // rust-lld doesn't search by default. Add it explicitly so the link
    // step finds `libraylib.so`.
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=raylib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    if std::env::var("DOCS_RS").is_ok() {
        std::fs::copy("bindings-for-docs-rs.rs", out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    } else {
        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
