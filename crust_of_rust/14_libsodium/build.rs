use std::env;
use std::path::PathBuf;

fn main() {
    let _ = pkg_config::Config::new()
        .print_system_libs(false)
        .atleast_version("1.0.20")
        .probe("libsodium")
        .unwrap();
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("sodium_init")
        .allowlist_function("crypto_generichash")
        .allowlist_var("crypto_generichash_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
