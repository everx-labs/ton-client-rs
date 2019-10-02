/*
 * Copyright 2018-2019 TON DEV SOLUTIONS LTD.
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

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::{Path, PathBuf};

use curl::easy::Easy;
use flate2::read::GzDecoder;

const BINARIES_URL: &str = "http://sdkbinaries.tonlabs.io";

fn main() {
    let out = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out);
    println!("cargo:rustc-link-lib=dylib=ton_client");

    install_binaries();
}

fn extract<P: AsRef<Path>, P2: AsRef<Path>>(archive_path: P, extract_to: P2) {
    let file = File::open(archive_path).unwrap();
    let mut unzipped = GzDecoder::new(file);
    let mut target_file = File::create(extract_to).unwrap();
    std::io::copy(&mut unzipped, &mut target_file).unwrap();
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

// Downloads and unpacks a prebuilt binary
fn install_binaries() {
    // Figure out the file names.
    let mut vec: Vec<&str> = env!("CARGO_PKG_VERSION").split(".").collect();
    let patch = u32::from_str_radix(&vec[2], 10).unwrap();
    let patch = format!("{}", patch - patch % 100);
    vec[2] = &patch;
    let version: String = vec.join("_");

    let files = if cfg!(target_os = "windows") {
        vec![
            (
                format!("tonclient_{}_win32_dll.gz", version),
                "ton_client.dll",
            ),
            (
                format!("tonclient_{}_win32_lib.gz", version),
                "ton_client.lib",
            ),
        ]
    } else if cfg!(target_os = "linux") {
        vec![(
            format!("tonclient_{}_linux.gz", version),
            "libton_client.so",
        )]
    } else if cfg!(target_os = "macos") {
        vec![(
            format!("tonclient_{}_darwin.gz", version),
            "libton_client.dylib",
        )]
    } else {
        panic!("Unknown target OS");
    };

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    for (file, target) in &files {
        download_file(&file, &out);
        extract(out.join(file), out.join(target));
    }
}
