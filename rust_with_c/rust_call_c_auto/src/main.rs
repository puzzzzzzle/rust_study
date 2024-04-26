mod c_libs;
// TODO 为啥main中要显示连接, lib中不用
#[link(name = "wrapper", kind = "static")]
extern "C" {}

fn main() {
    unsafe { println!("Hello, world! c zero {:?}", c_libs::zero()); }
}
