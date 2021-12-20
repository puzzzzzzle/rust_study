use ::function_name::named;

#[named]
pub fn hello() {
    // 普通输出
    println!("1 {}", 1);

    // 带 function_name 的输出
    println!(
        "hello_mod from: func: [{} fn] mod_path:  [{}]",
        function_name!(),
        module_path!()
    );
}
