extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/aes/aes_modes.c")
        .file("src/aes/aes_ni.c")
        .file("src/aes/aescrypt.c")
        .file("src/aes/aeskey.c")
        .file("src/aes/aestab.c")
        .file("src/aes/maple_encryption.c")
        .flag("-maes")
        .flag("-D__PROFILE_AES__")
        .flag("-Wno-unknown-pragmas")
        .compile("aes");

    let bindings = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .header("src/aes/maple_encryption.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
