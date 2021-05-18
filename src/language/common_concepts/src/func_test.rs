#[test]
fn func_test() {
    hello_func();
    hello_int(42);
    println!("hello by ret {}", hello_ret_int(40));
    println!("hello by test_if {}", test_if(true));
    println!("hello by test_if {}", test_if(false));
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

fn loop_test() {
    let mut arr = [0; 10];
    // loop
    {
        let mut index = 0;
        let end_index = loop {
            println!("loop {} {}", index, arr[index]);
            index += 1;
            if (index >= arr.len()) {
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
