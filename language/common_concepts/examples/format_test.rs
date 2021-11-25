use common::init_env;
use log::*;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
struct Hello {
    name: String,
}
// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    init_env().unwrap();
    let temp = Hello {
        name: "sss".to_string(),
    };
    info!("{:?}", temp);
    // Printing with `{:?}` is similar to with `{}`.
    info!("{:?} months in a year.", 12);
    // 数字指示第几个参数, 也可以用名字强制指定
    info!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    info!("Now {:?} will print!", Structure(3));
    info!("Now {} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    info!("Now {:#?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    info!("{:?}", peter);
    info!("{:#?}", peter);
}
