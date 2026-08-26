# libubpf-sys [![crates.io version number badge](https://img.shields.io/crates/v/libubpf-sys.svg)](https://crates.io/crates/libubpf-sys) [![Apache 2.0 License badge](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Rust bindings to _libubpf_ from IO Visor**

**Maintainer:** Alex Forster \<alex@alexforster.com\><br/>

_libubpf-sys_ is the packaged result of using _bindgen_ to automatically generate Rust FFI bindings to [
_libubpf_ from IO Visor](https://github.com/iovisor/ubpf).

## Features

By default, _libubpf-sys_ builds the vendored _ubpf_ source that ships with the crate.

The optional `custom-source` feature lets you build against your own _ubpf_ source tree instead of the vendored copy. Once enabling the feature, set the `LIBUBPF_SYS_SOURCE_DIR` environment variable to the path of a _ubpf_ source tree (the directory containing `vm/inc/ubpf.h`).

The optional `disable-retpolines` feature lets you potentially speed up your JIT'd code at the expense of having weaker Spectre mitigation.

The optional `1k-helper-functions` feature raises the max number of helper functions that can be registered from 64 to 1024.
