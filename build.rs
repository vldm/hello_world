extern crate syntex;
extern crate encoding_literals;

use std::env;
use std::path::Path;

fn main() {
    let mut registry = syntex::Registry::new();
    encoding_literals::plugin_registrar(&mut registry);

    let src = Path::new("src/main.rs");
    let dst = Path::new("src/_main.rs~");

    registry.expand("hello_world", &src, &dst).unwrap();
}
