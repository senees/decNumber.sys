/// Links to `The decNumber C Library` installed on Fedora 36.
fn main() {
  println!("cargo:rustc-link-lib=decNumber64");
  println!("cargo:rustc-link-search=/usr/lib64");
}
