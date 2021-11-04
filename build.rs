// SPDX-License-Identifier: Apache-2.0

//! Finds `libclang` static or dynamic libraries and links to them.
//!
//! # Environment Variables
//!
//! This build script can make use of several environment variables to help it
//! find the required static or dynamic libraries.
//!
//! * `LLVM_CONFIG_PATH` - provides a path to an `llvm-config` executable
//! * `LIBCLANG_PATH` - provides a path to a directory containing a `libclang`
//!    shared library or a path to a specific `libclang` shared library
//! * `LIBCLANG_STATIC_PATH` - provides a path to a directory containing LLVM
//!    and Clang static libraries

#![allow(unused_attributes)]

extern crate glob;

use std::path::Path;

#[path = "build/common.rs"]
pub mod common;
#[path = "build/dynamic.rs"]
pub mod dynamic;
#[path = "build/static.rs"]
pub mod static_;

/// Copy the file from the supplied source to the supplied destination.
#[cfg(feature = "runtime")]
fn copy(source: &str, destination: &Path) {
    use std::fs::File;
    use std::io::{Read, Write};

    let mut string = String::new();
    File::open(source)
        .unwrap()
        .read_to_string(&mut string)
        .unwrap();
    File::create(destination)
        .unwrap()
        .write_all(string.as_bytes())
        .unwrap();
}

/// Generates the finding and linking code so that it may be used at runtime.
#[cfg(feature = "runtime")]
fn main() {
    use std::env;

    if cfg!(feature = "static") {
        panic!("`runtime` and `static` features can't be combined");
    }

    let out = env::var("OUT_DIR").unwrap();
    copy("build/common.rs", &Path::new(&out).join("common.rs"));
    copy("build/dynamic.rs", &Path::new(&out).join("dynamic.rs"));
}

/// Finds and links to the required libraries.
#[cfg(not(feature = "runtime"))]
fn main() {
    if cfg!(feature = "static") {
        static_::link();
    } else {
        dynamic::link();
    }

    if let Some(output) = common::run_llvm_config(&["--includedir"]) {
        let directory = Path::new(output.trim_end());
        println!("cargo:include={}", directory.display());
    }
}
