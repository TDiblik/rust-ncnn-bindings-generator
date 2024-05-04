use std::{env, path::PathBuf};

fn main() {
    let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let ncnn_lib_dir = PathBuf::from(env::var("NCCN_LIB_DIR").unwrap());
    let ncnn_include_dir = PathBuf::from(env::var("NCCN_INCLUDE_DIR").unwrap());

    println!("cargo:rerun-if-env-changed=NCCN_LIB_DIR");
    println!("cargo:rerun-if-env-changed=NCCN_INCLUDE_DIR");

    let c_api_header = ncnn_include_dir.join("c_api.h");
    if !c_api_header.exists() {
        panic!(
            "c_api.h not found in {}",
            ncnn_include_dir.to_string_lossy()
        );
    }

    println!("cargo:rustc-link-search={}", ncnn_lib_dir.to_string_lossy());
    println!("cargo:rustc-link-lib=static=ncnn");

    if !cfg!(windows) {
        println!("cargo:rustc-link-lib=dylib=pthread");
    }

    let bindings = bindgen::Builder::default()
        .header(c_api_header.to_string_lossy())
        .allowlist_type("regex")
        .allowlist_function("ncnn.*")
        .allowlist_var("NCNN.*")
        .allowlist_type("ncnn.*")
        .generate()
        .expect("Unable to generate bindings :(");

    bindings
        .write_to_file(output_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
