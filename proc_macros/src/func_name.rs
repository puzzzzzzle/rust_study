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
use ::quote::{format_ident, quote_spanned};
use ::syn::{self, parse_macro_input, spanned::Spanned, Expr, ItemFn, Meta};

fn parse_invocation_macro_name(
    args: TokenStream,
    default_name: syn::Ident,
) -> syn::Result<syn::Ident> {
    if args.is_empty() {
        return Ok(default_name);
    }

    let meta = syn::parse::<Meta>(args)?;
    match meta {
        Meta::Path(path) => path.get_ident().cloned().ok_or_else(|| {
            syn::Error::new(
                path.span(),
                "expected identifier, e.g. #[func_name(my_macro)]",
            )
        }),
        Meta::NameValue(name_value) if name_value.path.is_ident("name") => match name_value.value {
            Expr::Path(expr_path) => expr_path.path.get_ident().cloned().ok_or_else(|| {
                syn::Error::new(
                    expr_path.path.span(),
                    "expected identifier in name = <ident>",
                )
            }),
            other => Err(syn::Error::new(
                other.span(),
                "expected identifier in name = <ident>",
            )),
        },
        _ => Err(syn::Error::new(
            meta.span(),
            "expected #[func_name], #[func_name(my_macro)] or #[func_name(name = my_macro)]",
        )),
    }
}

pub fn func_name(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析输入
    let mut input_fn = parse_macro_input!(input as ItemFn);

    // 获取函数名
    let fn_name = input_fn.sig.ident.to_string();
    let macro_name_for_call = match parse_invocation_macro_name(
        args,
        syn::Ident::new("func_name", input_fn.sig.ident.span()),
    ) {
        Ok(name) => name,
        Err(err) => return err.to_compile_error().into(),
    };
    let internal_macro_name = format_ident!("__{}_func_name_macro", input_fn.sig.ident);

    // println!("-------------- {}", ident);
    // 构造内部唯一宏, 避免与用户代码里的宏名冲突
    let expanded = quote::quote! {
    #[allow(unused_macros)]
    macro_rules! #internal_macro_name {
        () => {
           #fn_name
        };
    }

    #[allow(unused_macros)]
    macro_rules! #macro_name_for_call {
        () => {
            #internal_macro_name!()
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
