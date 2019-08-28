use std::env;

fn main() {
    let root = env!("CARGO_MANIFEST_DIR");
    println!("cargo:rustc-link-search=native={}", root);
    println!("cargo:rustc-link-lib=dylib=tonclient");
}
