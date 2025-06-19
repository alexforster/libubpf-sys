// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path;

fn main() {
    let src_dir = path::PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = path::PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let artifacts_dir = cmake::Config::new(src_dir.join("ubpf"))
        .always_configure(true)
        .define("UBPF_SKIP_EXTERNAL", "1")
        .profile(if cfg!(debug_assertions) {
            "Debug"
        } else {
            "RelWithDebugInfo"
        })
        .build_target("ubpf")
        .build();

    bindgen::Builder::default()
        .derive_default(true)
        .explicit_padding(true)
        .use_core()
        .default_enum_style(bindgen::EnumVariation::Consts)
        .prepend_enum_name(false)
        .layout_tests(false)
        .generate_comments(true)
        .generate_cstr(true)
        .emit_builtins()
        .merge_extern_blocks(true)
        .raw_line("pub type FILE = ::core::ffi::c_void;")
        .opaque_type("FILE")
        .blocklist_type("FILE")
        .allowlist_function("ubpf_.+")
        .allowlist_function("as_external_.+")
        .allowlist_type("ubpf_.+")
        .allowlist_var("ubpf_.+")
        .clang_arg(format!(
            "-I{}",
            artifacts_dir.join("build/vm").to_string_lossy()
        ))
        .header(src_dir.join("ubpf/vm/inc/ubpf.h").to_string_lossy())
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!(
        "cargo:rustc-link-search=native={}",
        artifacts_dir.join("build/lib").display()
    );
    println!("cargo:rustc-link-lib=static=ubpf");
}
