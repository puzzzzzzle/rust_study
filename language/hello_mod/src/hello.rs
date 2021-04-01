// curr in hello_mod mod
use ::function_name::named;

#[named]
pub fn hello() {
    println!("hello_mod from: func: [{} fn] mod_path:  [{}]", function_name!(),module_path!())
}