extern crate bindgen;

use std::env;
use std::path::PathBuf;

const DEFAULT_NFTABLES_INCLUDE_ARG: &str = "-I/usr/include/nftables";

fn main() {
    if let Ok(dir) = env::var("NFTABLES_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", dir);
    }
    // Tell cargo to tell rustc to link nftables
    // shared library.
    println!("cargo:rustc-link-lib=dylib=nftables");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default();
    let bindings = match env::var("NFTABLES_INCLUDE_DIR") {
        Ok(dir) => bindings.clang_arg(format!("-I{}", dir)),
        _       => bindings.clang_arg(DEFAULT_NFTABLES_INCLUDE_ARG)
    };
    let bindings = bindings
        // generate only nftables
        .allowlist_function("nft_.*")
        //.whitelist_type("nft_.*")
        .allowlist_type("nft_.*")
        .allowlist_var("NFT_CTX_.*")
        // do not bind functions using FILE
        .opaque_type("nft_ctx_set_output")
        .opaque_type("nft_ctx_set_error")
        // remove one extra type
        .blocklist_type("__uint32_t")
        // simplify constants names
        .prepend_enum_name(false)
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
