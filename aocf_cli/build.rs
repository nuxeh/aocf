use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");

    fs::write(&dest_path, format!("static PKG_VERSION: &str = \"{}\";\n", version)).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
