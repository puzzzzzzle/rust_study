use common;
use log::*;

#[test]
fn func_test() {
    common::init_env().unwrap();

    hello_func();
    hello_int(42);
    info!("hello by ret {}", hello_ret_int(40));
    info!("hello by test_if {}", test_if(true));
    info!("hello by test_if {}", test_if(false));
    let value = 33;
    info!("value is {value}");
    loop_test();
}
fn hello_func() {
    println!("hello func")
}
fn hello_int(name: i32) {
    println!("hello {}", name)
}

fn hello_ret_int(name: i32) -> i32 {
    let x = name;
    let y = {
        let z = x + 1;
        z + 1
    };
    y
}

fn test_if(cond: bool) -> i32 {
    let number = if cond { 42 } else { 45 };
    println!("get number {}", number);
    number
}
#[allow(unused_variables)]
fn loop_test() {
    let arr = [0; 10];
    // loop
    {
        let mut index = 0;
        let end_index = loop {
            println!("loop {} {}", index, arr[index]);
            index += 1;
            if index >= arr.len() {
                break index;
            }
        };
        println!("loop end {}", end_index);
    };
    // for
    {
        for item in arr.iter() {
            println!("for {}", item);
        }
    };
    // while
    {
        let mut index = 0;
        while index < arr.len() {
            println!("while {} {}", index, arr[index]);
            index += 1;
        }
    };
}
fn outter_fn() -> i32 {
    pub fn inner_fn() -> i32 {
        1
    }
    inner_fn()
}
#[test]
fn test_fn() {
    println!("{}", outter_fn());
}
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct Copyable {
    i1: i32,
    i2: i32,
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct NoCopyable {
    i1: i32,
    i2: i32,
}
#[test]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn closure_test() {
    let v1 = 1;
    // int 默认实现了copy trait, 所以捕获的时候使用了copy, 因此可以多次调用
    let fn_test = |x| x + v1;
    println!("{:?}", fn_test(4));
    println!("{:?}", fn_test(4));

    // String 没有实现 copy, 返回值有用到了 v1, 会直接take ownership 了, 导致 fn_test 之后, v1 不可用了, fn_test 也只能使用一次
    let v1 = "s1".to_string();
    // 因为返回值用到了v1, 导致take ownership  了, string 的 Add 操作符, 是直接在第一个string上调push_str的
    let fn_test = |x| v1 + "  ... called";
    // println!("{}", v1);  // ERROR
    println!("{}", fn_test(4));
    // println!("{}", fn_test(4));  // ERROR

    // 同理, 如果闭包中不需要所有权, 那么久不用take
    let v1 = "s1".to_string();
    let fn_test = |x| v1.clone() + "  ... called";
    println!("{}", v1);
    println!("{}", fn_test(4));
    println!("{}", fn_test(4));

    // Copyable 实现了 Copy, 就可以支持多次使用了, 应为每次take ownership 时, 没有真的take, 都是copy
    let mut v1 = Copyable { i1: 42, i2: 666 };
    let fn_test = |x| v1;
    println!("{:?}", v1);
    println!("{:?}", fn_test(4));
    println!("{:?}", fn_test(4));

    // 即使使用了 move 标记, 也是如此
    // 主要用于闭包会脱离当前环境时, 推断可能会出问题的情况
    let v1 = Copyable { i1: 42, i2: 666 };
    let fn_test = move |x| v1;
    println!("{:?}", v1);
    println!("{:?}", fn_test(4));
    println!("{:?}", fn_test(4));

    // NoCopyable 未实现 Copy, 只能调一次
    let mut v1 = NoCopyable { i1: 42, i2: 666 };
    let fn_test = |x| v1;
    // 在这之后 v1 不可用了
    // println!("{:?}", v1);   // ERROR
    println!("{:?}", fn_test(4));
    // println!("{:?}", fn_test(4));   // ERROR
}

#[test]
fn destructuring_test() {
    let mut p = (1, 2);
    println!("{:?}", p);
    let (ref mut first, ref mut last) = p;
    *first = 200;
    *last *= 1000;
    println!("{:?}", p);
    match p {
        (100, _) => println!("first 100"),
        (_, 2000) => println!("second 2000"),

        _ => println!("other case {:?}", p),
    }
}
fn main() {}
