use common_macros::trace_var;
#[trace_var(p, n)]
fn factorial(mut n: u64) -> u64 {
    let mut p = 1;
    while n > 1 {
        p *= n;
        n -= 1;
    }
    p
}
#[test]
fn test_trace_var() {
    println!("{}", factorial(8));
}

macro_rules! say_hello {
    () => {
        println!("hello")
    };
}

fn main() {
    say_hello!();
}
