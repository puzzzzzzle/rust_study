use proc_macros::func_name;
#[test]
#[func_name]
fn func_name_test3() {
    println!("this func name is {}", func_name!());
    assert_eq!("func_name_test3", func_name!())
}

#[func_name]
fn func_name_test4() {
    println!("this func name is {}", func_name!());
}
fn main() {}
