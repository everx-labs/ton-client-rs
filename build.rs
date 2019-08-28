//use std::env;

fn main() {
    println!("cargo:rustc-link-search=native={}", "/Users/michaelvlasov/projects/ton/TON-SDK/target/release");
    println!("cargo:rustc-link-lib=dylib=tonclient");
}
