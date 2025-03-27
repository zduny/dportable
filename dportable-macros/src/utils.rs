use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Plus,
    Item, TypeParamBound, WherePredicate,
};

pub struct Items(Vec<Item>);

impl Parse for Items {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(Items(items))
    }
}

pub fn create_non_sync_send_variant_for_wasm(mut items: Items) -> TokenStream {
    let mut output_items = vec![];

    for item in items.0.drain(..) {
        match item {
            syn::Item::Trait(mut item) => {
                output_items.push(quote! {
                    #[cfg(not(target_arch = "wasm32"))]
                    #item
                });

                remove_send(&mut item.supertraits);
                output_items.push(quote! {
                    #[cfg(target_arch = "wasm32")]
                    #item
                });
            }
            syn::Item::Impl(mut item) => {
                output_items.push(quote! {
                    #[cfg(not(target_arch = "wasm32"))]
                    #item
                });

                if let Some(where_clause) = &mut item.generics.where_clause {
                    for predicate in where_clause.predicates.iter_mut() {
                        if let WherePredicate::Type(predicate) = predicate {
                            remove_send(&mut predicate.bounds);
                        }
                    }
                }
                output_items.push(quote! {
                    #[cfg(target_arch = "wasm32")]
                    #item
                });
            }
            _ => output_items.push(quote! { #item }),
        }
    }

    quote! {
        #(#output_items)*
    }
}

fn remove_send(punctuated: &mut Punctuated<TypeParamBound, Plus>) {
    let mut result = Punctuated::new();
    for bound in punctuated.iter().filter(|bound| !is_sync_or_send(bound)) {
        result.push(bound.clone());
    }
    *punctuated = result;
}

fn is_sync_or_send(bound: &TypeParamBound) -> bool {
    if let TypeParamBound::Trait(bound) = bound {
        bound.path.is_ident("Sync") || bound.path.is_ident("Send")
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::{create_non_sync_send_variant_for_wasm, Items};

    #[test]
    fn test_create_non_sync_send_variant_for_wasm() {
        let expected = quote! {
            #[cfg(not(target_arch = "wasm32"))]
            /// Some docs
            pub trait SomeTrait: B + Send + C + 'static {}

            #[cfg(target_arch = "wasm32")]
            /// Some docs
            pub trait SomeTrait: B + C + 'static {}

            #[cfg(not(target_arch = "wasm32"))]
            impl<T> SomeTrait for T where T: B + Send + C + 'static {}

            #[cfg(target_arch = "wasm32")]
            impl<T> SomeTrait for T where T: B + C + 'static {}
        };

        let input = quote! {
            /// Some docs
            pub trait SomeTrait: B + Send + C + 'static {}

            impl<T> SomeTrait for T where T: B + Send + C + 'static {}
        };

        let input = syn::parse2::<Items>(input).unwrap();

        let tokens = create_non_sync_send_variant_for_wasm(input);
        let actual = quote! {
            #tokens
        };

        assert_eq!(expected.to_string(), actual.to_string());
    }
}
