extern crate prost_build;

fn main() {
  println!("cargo:rerun-if-changed=src/items.proto");
  prost_build::compile_protos(&["src/items.proto"], &["src/"]).unwrap();
}
