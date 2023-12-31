use cmake::Config;

fn main() -> miette::Result<()> {

  let dst = Config::new("cpp").build();

  println!("cargo:rustc-link-search=native={}", dst.display());
  println!("cargo:rustc-link-lib=rust-cpp-cmake-bindings-cpp");

  let inc_path = std::path::PathBuf::from("cpp/include");
  let mut b = autocxx_build::Builder::new("src/main.rs", &[&inc_path]).build()?;

  b.flag_if_supported("-std=c++20")
   .compile("rust-cpp-cmake-bindings");
  println!("cargo:rerun-if-changed=src/main.rs");
  Ok(())
}
