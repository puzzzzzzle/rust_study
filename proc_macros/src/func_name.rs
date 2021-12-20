//! 对一个函数, 添加 function name 的宏
// 思路:
// 对每个函数, 内部定义一个 macro_rules! func_name 的宏, 函数中直接使用这个宏, 不同的函数中, 由于命名空间不同, 所以不会冲突
// #[test]
// fn func_name_test() {
//     macro_rules! func_name {
//         () => {
//             "func_name_test"
//         };
//     }
//     println!("this func name is {}", func_name!())
// }
// #[test]
// fn func_name_test1() {
//     macro_rules! func_name {
//         () => {
//             "func_name_test1"
//         };
//     }
//     println!("this func name is {}", func_name!())
// }

extern crate proc_macro;
use ::proc_macro::TokenStream;
use ::quote::quote_spanned;
use ::syn::{self, parse_macro_input, spanned::Spanned, ItemFn};

pub fn func_name(_: TokenStream, input: TokenStream) -> TokenStream {
    // 解析输入
    let mut input_fn = parse_macro_input!(input as ItemFn);

    // 获取函数名
    let ident = input_fn.sig.ident.to_string();

    // println!("-------------- {}", ident);
    // 构造 func_name 宏
    let expanded = quote::quote! {
    #[allow(unused_macros)]
    macro_rules! func_name {
        () => {
           #ident
        };
    }
    };

    // 把 func_name 宏 插入到函数体中
    let block = *input_fn.block;
    *input_fn.block = syn::parse_quote! {
        {
            #expanded
            #block
        }
    };

    // 返回stream
    TokenStream::from(quote_spanned! { input_fn.span() =>
        #input_fn
    })
}
