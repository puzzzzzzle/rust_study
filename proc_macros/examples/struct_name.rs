use proc_macros::StructName;
use proc_macros_trait::StructName;

#[derive(StructName)]
struct Empty {}

impl Empty {
    pub fn hello() -> String {
        format!("hello from {}", Empty::struct_name())
    }
    pub fn hello_self(&self) -> String
    {
        format!("hello self from {}", Empty::struct_name())
    }
}

#[derive(StructName)]
struct HaHaHa {}

fn main() {
    println!("{}", Empty::hello());
    println!("{}", Empty{}.hello_self());


    println!("{}", Empty::struct_name());
    println!("{}", HaHaHa::struct_name());
}
