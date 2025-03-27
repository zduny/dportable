mod test;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn dtest_configure(_tokens: TokenStream) -> TokenStream {
    test::dtest_configure().into()
}

#[proc_macro_attribute]
pub fn dtest(_attr: TokenStream, item: TokenStream) -> TokenStream {
    test::dtest(item.into()).into()
}

#[proc_macro]
pub fn create_non_sync_send_variant_for_wasm(tokens: TokenStream) -> TokenStream {
    utils::create_non_sync_send_variant_for_wasm(syn::parse_macro_input!(tokens)).into()
}
