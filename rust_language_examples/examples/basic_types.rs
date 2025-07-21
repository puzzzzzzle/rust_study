use std::fmt;

#[derive(Debug)]
struct  Printable(i32);
impl fmt::Display for Printable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value is : {}", self.0)
    }
}

#[test]
fn test_fmt() {
    let p = Printable(10);
    assert_eq!(format!("{}", p), "value is : 10");
    assert_eq!(format!("{:?}", p), "Printable(10)");
    println!("{}", p);
    println!("{:?}", p); // debug
}

#[test]
fn test_basic_types() {
    let x = 5;
    let x = 5f32; // float
    let x = ("sss",42i128);
    
}

fn main() {}