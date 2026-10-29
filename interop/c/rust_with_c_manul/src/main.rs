use rust_call_c_manul::{c_get_pid, c_one, c_zero_safe, get_rs_value_safe};
use stopwatch::Stopwatch;

fn main() {
    let mut sw = Stopwatch::start_new();
    println!("Hello, world!");
    println!("--   time use : {:?}", sw.elapsed());
    sw.restart();
    println!("call  c {}", c_zero_safe());
    println!("--   time use : {:?}", sw.elapsed());
    sw.restart();
    println!("call  c {}", unsafe { c_one() });
    println!("--   time use : {:?}", sw.elapsed());
    sw.restart();
    println!("c call call  rs {}", get_rs_value_safe());
    println!("--   time use : {:?}", sw.elapsed());
    sw.restart();
    println!("c call call  pid {}", c_get_pid());
    println!("--   time use : {:?}", sw.elapsed());
}
