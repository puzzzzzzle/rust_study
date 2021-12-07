// use crate::trace_var;
use proc_macros::trace_var;
#[test]
#[trace_var(p, n)]
fn factorial() {
    let mut n = 10;
    let mut p = 1;
    while n > 1 {
        p *= n;
        n -= 1;
    }
}