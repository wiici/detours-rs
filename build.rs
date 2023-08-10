use std::{env, path::PathBuf, str::FromStr};

use bindgen;
use cmake::Config;

fn main() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from_str(env::var("OUT_DIR").unwrap().as_str())
        .expect("Cannot get OUT_DIR env var");

    println!("cargo:rerun-if-changed={}", file!());

    // Build Detours with CMake that will install lib + headers in OUT_DIR by default
    _ = Config::new("libdetours").build();

    println!("cargo:rustc-link-lib=libdetours");
    println!("cargo:rustc-link-search={}", out_dir.join("lib").display());

    let should_generate_bindings = env::var("GENERATE_DETOURS_BINDINGS").is_ok();
    if should_generate_bindings {
        let detours_header_include_dir = out_dir.join("include");

        let bindings = bindgen::Builder::default()
            .allowlist_function("DetourTransactionBegin")
            .allowlist_function("DetourUpdateThread")
            .allowlist_function("DetourAttach")
            .allowlist_function("DetourDetach")
            .allowlist_function("DetourTransactionCommit")
            .header("libdetours/detours_wrapper.h")
            .clang_arg(format!("-I{}", detours_header_include_dir.display()))
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Couldn't generate bindings");

        bindings
            .write_to_file(manifest_dir.join("src/bindings.rs"))
            .expect("Couldn't write bindings");
    }
}
