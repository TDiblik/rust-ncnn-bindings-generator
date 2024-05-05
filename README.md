# Rust NCNN Bindings Generator

This project creates Rust bindings for the [ncnn (by Tencent)](https://github.com/Tencent/ncnn) using the `c_api.h`. This project assumes that you compiled `ncnn` yourself and you want to statically link it.

# How to use

First of all, make sure to compile `ncnn` on your system and set the following environment variables:

```bash
NCNN_LIB_DIR='path to a directory containing ncnn.lib' # eg. C:\src\vcpkg\installed\x64-windows-static-md\lib
NCNN_INCLUDE_DIR='path to a directory containing c_api.h (usually $(COMPILATION_PATH)/include/ncnn)' # eg. C:\src\vcpkg\installed\x64-windows-static-md\include\ncnn
```

You can either set them globally on your system, or prepend them before the `cargo r` command (that's what I prefer).

Then modify your `Cargo.toml`:

```toml
ncnn-bindings = { git = "https://github.com/TDiblik/rust-ncnn-bindings-generator" }
```

Once you've done that, you're ready to go. If you're developing inside VSCode, you probably want to create `.vscode/settings.json` and insert the following there:

```json
{
  ...
  "rust-analyzer.cargo.extraEnv": {
    "NCNN_LIB_DIR": "same_as_the_env_variable",
    "NCNN_INCLUDE_DIR": "same_as_the_env_variable"
  }
  ...
}
```

This ensures that your `rust-analyzer` extension compiles your project successfully.

And you're ready to go! You can test that everything works correctly with:

```rust
fn main() {
    let ncnn_version_c_buf = unsafe { ncnn_bindings::ncnn_version() };
    let ncnn_version_c_str = unsafe { CStr::from_ptr(ncnn_version_c_buf) };
    let ncnn_version: &str = ncnn_version_c_str.to_str().unwrap();
    println!("{ncnn_version}");
}
```

Now, you can use the API as you would in `C++`. This project has no intention in making a safe wrapper around the `C API`.
