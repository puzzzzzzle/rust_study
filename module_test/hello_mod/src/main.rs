mod hello;
mod hello_dir;
mod hello_sub;

use ::function_name::named;
use guess_game;

// use crate::guess_game;

#[named]
fn test() -> i32 {
    println!(
        "hello_mod from: func: [{} fn] mod_path:  [{}]",
        function_name!(),
        module_path!()
    );
    return 42;
}

#[named]
fn test_1() -> i32 {
    println!(
        "hello_mod from: func: [{} fn] mod_path:  [{}]",
        function_name!(),
        module_path!()
    );
    42
}

#[named]
fn test_2() {
    // return void
    println!(
        "hello_mod from: func: [{} fn] mod_path:  [{}]",
        function_name!(),
        module_path!()
    );
}

fn main() {
    let ret = test();
    println!("{}", ret);
    let ret = test_1();
    println!("{}", ret);

    let _ret = test_2();
    // println!("{}", ret);

    hello::hello();
    // error
    // hello_sub::hello_kk::hello_pub::hello_private();
    hello_sub::hello_kk::hello_pub::hello();
    hello_dir::hello();
    hello_dir::hello_dir_1::hello();
    guess_game::guess::guess_game();
    // guess_game::guess_game();
    // error
    // hello_dir::hello_dir_2::hello_mod();
}
