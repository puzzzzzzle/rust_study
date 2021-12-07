//! test trace var proc mac
//! 测试 trace var 宏

use proc_macros::trace_var;


/// 会在执行时, 对任何p n 的赋值操作, 打印状态
///
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