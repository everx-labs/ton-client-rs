use std::env;

fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    println!("cargo:rustc-link-search=native={}/libs", root);
    println!("cargo:rustc-link-lib=dylib=tonclient");
}
