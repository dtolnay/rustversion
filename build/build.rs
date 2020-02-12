mod rustc;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::{self, Command};

fn main() {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let output = match Command::new(&rustc)
        .args(&["--version", "--verbose"])
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            let rustc = rustc.to_string_lossy();
            eprintln!(
                "Error: failed to run `{} --version --verbose`: {}",
                rustc, e
            );
            process::exit(1);
        }
    };

    let string = match String::from_utf8(output.stdout) {
        Ok(string) => string,
        Err(e) => {
            let rustc = rustc.to_string_lossy();
            eprintln!(
                "Error: failed to parse output of `{} --version`: {}",
                rustc, e,
            );
            process::exit(1);
        }
    };

    let version = match rustc::parse(&string) {
        Some(version) => format!("{:#?}\n", version),
        None => {
            eprintln!(
                "Error: unexpected output from `rustc --version`: {:?}\n\n\
                 Please file an issue in https://github.com/dtolnay/rustversion",
                string
            );
            process::exit(1);
        }
    };

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let out_file = Path::new(&out_dir).join("version.rs");
    fs::write(out_file, version).expect("failed to write version.rs");
}
