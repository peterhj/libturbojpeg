use std::env;
use std::fs::{create_dir_all};
use std::path::{PathBuf};
use std::process::{Command};

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let out_dir = env::var("OUT_DIR").unwrap();

  let cc = env::var("CC").unwrap_or(format!("gcc"));
  let cxx = env::var("CXX").unwrap_or(format!("g++"));

  let mut jpegturbo_lib_dst_path = PathBuf::from(&out_dir);
  jpegturbo_lib_dst_path.push("libjpegturbo.a");

  if !jpegturbo_lib_dst_path.exists() {
    let mut jpegturbo_src_path = PathBuf::from(&manifest_dir);
    jpegturbo_src_path.push("libjpeg-turbo");
    let mut jpegturbo_build_path = PathBuf::from(&out_dir);
    jpegturbo_build_path.push("libjpeg-turbo-build");

    create_dir_all(&jpegturbo_build_path).ok();

    // autoreconf -fiv
    Command::new("autoreconf")
      .current_dir(&jpegturbo_src_path)
      .arg("-fiv")
      .status().unwrap();

    let mut configure_path = PathBuf::from(&jpegturbo_src_path);
    configure_path.push("configure");
    // CC=gcc-4.9 CXX=g++-4.9 ../libjpeg-turbo-1.4.90/configure --prefix=`pwd` --enable-static=yes --with-pic=yes --with-jpeg8 --with-simd
    Command::new(configure_path)
      .current_dir(&jpegturbo_build_path)
      .env("CC",  &cc)
      .env("CXX", &cxx)
      .arg("--enable-static=yes")
      .arg("--with-pic=yes")
      .arg("--with-jpeg8")
      .arg("--with-simd")
      .status().unwrap();

    Command::new("make")
      .current_dir(&jpegturbo_build_path)
      .status().unwrap();

    assert!(Command::new("cp")
      .arg(&jpegturbo_build_path.join(".libs").join("libturbojpeg.a"))
      .arg(&jpegturbo_lib_dst_path)
      .status().unwrap()
      .success());
  }

  println!("cargo:rustc-link-search=native={}", out_dir);
}
