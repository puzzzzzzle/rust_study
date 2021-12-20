//! this file is used for holding macros defined by me.
//! proc_macro_attribute and it's func can only appear in lib.rs at the root of crate,
//! so ues this file to hole all the procedural macros

mod func_name;
mod struct_name;
mod trace_var;

use proc_macro::TokenStream;

/// 过程宏 - 熟悉宏
/// Attribute to print the value of the given variables each time they are
/// reassigned.
/// 对指定的变量的任意改变操作, 打印日志
///
#[proc_macro_attribute]
pub fn trace_var(args: TokenStream, input: TokenStream) -> TokenStream {
    trace_var::trace_var(args, input)
}

#[proc_macro_derive(StructName)]
pub fn struct_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    struct_name::struct_name(input)
}

#[proc_macro_attribute]
pub fn func_name(params: TokenStream, input: TokenStream) -> TokenStream {
    func_name::func_name(params, input)
}
