mod test;

use proc_macro::TokenStream;

#[proc_macro]
pub fn dtest_configure(_tokens: TokenStream) -> TokenStream {
    test::dtest_configure().into()
}

#[proc_macro_attribute]
pub fn dtest(_attr: TokenStream, item: TokenStream) -> TokenStream {
    test::dtest(item.into()).into()
}