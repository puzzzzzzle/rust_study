extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/c_wrapper.c")
        .compile("c_wrapper.a");
}