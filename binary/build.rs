use std::{fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir).join("./adder.rs");

    let source_file = include_str!("./src/main.rs");

    // regex extract type from library::adder_value_type!(TYPE);
    let re = regex::Regex::new(r"api_type!\((.+)\)").unwrap();
    let capture = re
        .captures(source_file)
        .and_then(|captures| captures.get(1));

    let adder_value_type = if let Some(capture) = capture {
        capture.as_str()
    } else {
        "i32"
    };

    let proxy_code = format!("type AdderType = {};", adder_value_type);
    fs::write(&out_path, proxy_code).unwrap();

    println!("cargo:warning=binary build path={:?}", out_path);
}
