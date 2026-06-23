// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path;

const SOURCE_DIR_ENV: &str = "LIBUBPF_SYS_SOURCE_DIR";

fn main() {
    let src_dir = path::PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = path::PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // With the `custom-source` feature enabled, build the ubpf source tree pointed
    // to by `LIBUBPF_SYS_SOURCE_DIR` instead of the vendored `ubpf/` submodule.
    let ubpf_dir = if cfg!(feature = "custom-source") {
        println!("cargo:rerun-if-env-changed={}", SOURCE_DIR_ENV);
        let source_dir = env::var_os(SOURCE_DIR_ENV).unwrap_or_else(|| {
            panic!(
                "the `custom-source` feature is enabled but the `{}` environment \
                 variable is not set; set it to the path of a ubpf source tree",
                SOURCE_DIR_ENV
            )
        });
        let source_dir = path::PathBuf::from(source_dir);
        if !source_dir.join("vm/inc/ubpf.h").is_file() {
            panic!(
                "`{}` ({}) does not look like a ubpf source tree: \
                 expected to find `vm/inc/ubpf.h`",
                SOURCE_DIR_ENV,
                source_dir.display()
            );
        }
        source_dir
    } else {
        src_dir.join("ubpf")
    };

    let artifacts_dir = cmake::Config::new(&ubpf_dir)
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
        .header(ubpf_dir.join("vm/inc/ubpf.h").to_string_lossy())
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
