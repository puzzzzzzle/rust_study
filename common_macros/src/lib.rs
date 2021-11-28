mod trace_var;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn trace_var(args: TokenStream, input: TokenStream) -> TokenStream {
    trace_var::trace_var(args, input)
}
