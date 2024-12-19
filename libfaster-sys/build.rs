extern crate bindgen;

use cmake::Config;
use std::{
  env,
  fs,
  path::{Path, PathBuf},
};

// Credit to: https://github.com/rust-rocksdb/rust-rocksdb/blob/master/librocksdb-sys/build.rs
fn fail_on_empty_directory(name: &str) {
  if fs::read_dir(name).unwrap().count() == 0 {
    println!(
      "The `{}` directory is empty, did you forget to pull the submodules?",
      name
    );
    println!("Try `git submodule update --init --recursive`");
    panic!();
  }
}

fn faster_bindgen() {
  let faster_path = fs::canonicalize(Path::new("../libfaster-sys/FASTER/cc/src")).unwrap();

  let bindings = bindgen::Builder::default()
        .header("faster-c.h")
        // sudo apt-get install libc++-dev g++-12
        // https://microsoft.github.io/FASTER/docs/fasterkv-cpp/
        .clang_args([
          "-x".into(), "c++".into(),
          "-std=c++14".into(),
          format!("-I{}", faster_path.display()),
        ])
        .opaque_type("std::.*")
        // https://github.com/rust-lang-nursery/rust-bindgen/issues/550
        .blocklist_type("max_align_t")
        // https://github.com/rust-lang/rust-bindgen/issues/1848
        .ctypes_prefix("libc")
        .enable_cxx_namespaces()
        .generate()
        .expect("unable to generate faster bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("unable to write faster bindings");
}

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=FASTER/");

  fail_on_empty_directory("FASTER");

  faster_bindgen();

  let dst = Config::new("FASTER/cc").cflag("--std=c++11 ").build();

  println!("cargo:rustc-link-search=native={}/{}", dst.display(), "build");
  // Fix this...
  println!("cargo:rustc-link-lib=static=faster");
  println!("cargo:rustc-link-lib=stdc++fs");
  println!("cargo:rustc-link-lib=uuid");
  println!("cargo:rustc-link-lib=tbb");
  println!("cargo:rustc-link-lib=gcc");
  println!("cargo:rustc-link-lib=stdc++");
  println!("cargo:rustc-link-lib=aio");
  println!("cargo:rustc-link-lib=pthread");
  println!("cargo:rustc-link-lib=m");
}
