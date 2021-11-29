use common_macros::*;

#[trace_var(p, n)]
fn factorial(mut n: u64) -> u64 {
    let mut p = 1;
    while n > 1 {
        p *= n;
        n -= 1;
    }
    p
}

fn main() {
    factorial(10);
}
