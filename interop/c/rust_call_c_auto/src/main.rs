mod c_libs;
// main 中使用, 记得在 build.rs 中声明链接   println!("cargo:rustc-link-lib=wrapper");

fn main() {
    unsafe { println!("Hello, world! c zero {:?}", c_libs::zero()); }
}
