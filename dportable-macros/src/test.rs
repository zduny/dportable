use proc_macro2::TokenStream;
use quote::quote;

pub fn dtest_configure() -> TokenStream {
    quote! {
        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen_test::wasm_bindgen_test_configure;
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_test_configure!(run_in_browser);
    }
}

pub fn dtest(item: TokenStream) -> TokenStream {
    if let Ok(mut item) = syn::parse2::<syn::ItemFn>(item) {
        item.attrs
            .retain(|attribute| !is_dnet_test_attribute(attribute));
        let output = quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #[::tokio::test]
            #item

            #[cfg(target_arch = "wasm32")]
            #[::wasm_bindgen_test::wasm_bindgen_test]
            #item
        };

        output
    } else {
        panic!("expected a function");
    }
}

fn is_dnet_test_attribute(attribute: &syn::Attribute) -> bool {
    attribute.path().is_ident("dnet_test")
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::dtest;

    #[test]
    fn test_skip_self() {
        let expected = quote! {
            #[cfg(not(target_arch = "wasm32"))]
            #[::tokio::test]
            async fn some_test(&self, a: u32, b: String) {}

            #[cfg(target_arch = "wasm32")]
            #[::wasm_bindgen_test::wasm_bindgen_test]
            async fn some_test(&self, a: u32, b: String) {}
        };

        let test = quote! {
            #[dnet_test]
            async fn some_test(&self, a: u32, b: String) {}
        };

        let item = dtest(test);

        let actual = quote! {
            #item
        };

        assert_eq!(expected.to_string(), actual.to_string());
    }
}
