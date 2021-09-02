use rust_call_c_manul::{c_zero_safe, get_rs_value_safe};

fn main() {
    println!("Hello, world!");
    println!("call  c {}", c_zero_safe());
    println!("c call call  rs {}", get_rs_value_safe());
}
