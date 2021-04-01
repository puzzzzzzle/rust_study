mod hello;
mod hello_sub;
mod hello_dir;

fn main() {
    hello::hello();
    hello_sub::hello_kk::hello_pub::hello();
    // error
    // hello_sub::hello_kk::hello_pub::hello_private();

    println!("Hello, world!");
}
