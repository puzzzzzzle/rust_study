use proc_macros::StructName;
use proc_macros_trait::StructName;

#[derive(StructName)]
struct Empty {}

impl Empty {
    pub fn hello() -> String {
        "hello from empty".to_string()
    }
}

#[derive(StructName)]
struct HaHaHa {}
fn main() {
    println!("{}", Empty::hello());
    // let t = Empty {};
    // println!("{}", t.Hello());

    println!("{}", Empty::struct_name());
    println!("{}", HaHaHa::struct_name());
}
