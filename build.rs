use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let output = Command::new(rustc)
        .arg("--version")
        .output()
        .expect("Failed to exec rustc");

    let string = String::from_utf8(output.stdout).expect("rustc output not utf8");
    let dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");

    fs::write(&Path::new(&dir).join("version.txt"), &string).expect("failed to write version.txt")
}
