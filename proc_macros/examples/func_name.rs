use proc_macros::func_name;
#[test]
#[func_name]
fn func_name_test3() {
    println!("this func name is {}", func_name!());
    assert_eq!("func_name_test3", func_name!())
}
#[test]
#[func_name(current_name)]
fn func_name_test4() {
    macro_rules! func_name {
        () => {
            "local_macro_name"
        };
    }

    println!("local macro result is {}", func_name!());
    println!("generated func name is {}", current_name!());
    assert_eq!("local_macro_name", func_name!());
    assert_eq!("func_name_test4", current_name!());
}
fn main() {}
