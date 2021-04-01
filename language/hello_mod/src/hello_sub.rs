pub mod hello_kk {
    pub mod hello_pub {
        use ::function_name::named;

        #[named]
        pub fn hello() {
            println!("hello_mod from: func: [{} fn] mod_path:  [{}]", function_name!(), module_path!())
        }

        #[named]
        fn hello_private() {
            println!("hello_mod from: func: [{} fn] mod_path:  [{}]", function_name!(), module_path!())
        }
    }

    mod hello_private {}
}