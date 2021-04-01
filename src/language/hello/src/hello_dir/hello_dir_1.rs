use ::function_name::named;

#[named]
pub fn hello() {
    println!("hello from: func: [{} fn] mod_path:  [{}]", function_name!(),module_path!())
}