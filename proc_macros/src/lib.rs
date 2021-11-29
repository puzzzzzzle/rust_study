//! this file is used for holding macros defined by me.
//! proc_macro_attribute and it's func can only appear in lib.rs at the root of crate,
//! so ues this file to hole all the procedural macros

mod trace_var;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn trace_var(args: TokenStream, input: TokenStream) -> TokenStream {
    trace_var::trace_var(args, input)
}
