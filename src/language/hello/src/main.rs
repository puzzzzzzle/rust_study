mod hello;
mod hello_sub;
mod hello_dir;

use ::function_name::named;

#[named]
fn test() {
    println!("hello from: func: [{} fn] mod_path:  [{}]", function_name!(), module_path!())
}

fn main() {
    test();
    hello::hello();
    // error
    // hello_sub::hello_kk::hello_pub::hello_private();
    hello_sub::hello_kk::hello_pub::hello();
    hello_dir::hello();
    hello_dir::hello_dir_1::hello();
    // error
    // hello_dir::hello_dir_2::hello();
}
