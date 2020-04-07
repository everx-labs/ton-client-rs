/*
 * Copyright 2018-2020 TON DEV SOLUTIONS LTD.
 *
 * Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
 * this file except in compliance with the License.  You may obtain a copy of the
 * License at: https://ton.dev/licenses
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific TON DEV software governing permissions and
 * limitations under the License.
 */

extern crate curl;
extern crate flate2;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::io::Write;

use flate2::read::GzDecoder;
use curl::easy::Easy;
use cargo_toml::Manifest;

const BINARIES_URL: &str = "http://sdkbinaries.tonlabs.io";

fn main() {
    let out = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out);
    println!("cargo:rustc-link-lib=dylib=ton_client");

    install_binaries();
}

fn extract<P: AsRef<Path>, P2: AsRef<Path>>(archive_path: P, extract_to: P2) {
    if !extract_to.as_ref().exists() {
        let file = File::open(archive_path).unwrap();
        let mut unzipped = GzDecoder::new(file);
        let mut target_file = File::create(extract_to).unwrap();
        std::io::copy(&mut unzipped, &mut target_file).unwrap();
    }
}

fn download_file(file_name: &str, download_dir: &PathBuf) {
    let binary_url = format!("{}/{}", BINARIES_URL, file_name);

    let file_name = download_dir.join(file_name);

    if !file_name.exists() {
        let f = File::create(&file_name).unwrap();
        let mut writer = BufWriter::new(f);
        let mut easy = Easy::new();
        easy.url(&binary_url).unwrap();
        easy.write_function(move |data| Ok(writer.write(data).unwrap()))
            .unwrap();
        easy.perform().unwrap();

        let response_code = easy.response_code().unwrap();
        if response_code != 200 {
            panic!(
                "Unexpected response code {} for {}",
                response_code, binary_url
            );
        }
    }
}

#[derive(Deserialize)]
struct Metadata {
    binaries_version: String
}

// Downloads and unpacks a prebuilt binary
fn install_binaries() {
    // Take binaries version from manifest.
    let manifest: Manifest<Metadata> = cargo_toml::Manifest::from_path_with_metadata(
        env!("CARGO_MANIFEST_DIR").to_owned() + "/Cargo.toml").expect("Can not read manifest");
    let dotted_version: String = manifest.package.expect("No package section")
        .metadata.expect("No metadata")
        .binaries_version;

    let version = dotted_version.replace(".", "_");

    let files = if cfg!(target_os="windows") {
        vec![
            (format!("tonclient_{}_win32_dll.gz", version), "ton_client.dll"),
            (format!("tonclient_{}_win32_lib.gz", version), "ton_client.lib")
        ]
    } else if cfg!(target_os="linux") {
        vec![(format!("tonclient_{}_linux.gz", version), "libton_client.so")]
    } else if cfg!(target_os="macos") {
        vec![(format!("tonclient_{}_darwin.gz", version), "libton_client.dylib")]
    } else {
        panic!("Unknown target OS");
    };

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("Downloading binaries with version {}", dotted_version);

    for (file, target) in &files {
        download_file(&file, &out);
        extract(out.join(file), out.join(target));
    }
}