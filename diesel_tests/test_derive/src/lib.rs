extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, token::Async, ItemFn};

#[proc_macro_attribute]
pub fn test(_: TokenStream, item: TokenStream) -> TokenStream {
    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = parse_macro_input!(item as ItemFn);

    let wasm_attr = quote! {
        #[cfg(all(target_family = "wasm", not(target_os = "wasi")))]
        #[wasm_bindgen_test::wasm_bindgen_test]
    };
    let native_attr = quote! {
        #[cfg(not(all(target_family = "wasm", not(target_os = "wasi"))))]
        #[test]
    };
    
    let wasm_prepare = quote! {
        #[cfg(all(target_family = "wasm", not(target_os = "wasi"), feature = "sqlite"))]
        crate::init_sqlite().await.unwrap();
    };

    let native_sig= sig.clone();
    let mut wasm_sig= sig.clone();
    wasm_sig.asyncness = Some(Async::default());

    quote!(
        #wasm_attr
        #(#attrs)*
        #vis #wasm_sig {
            #wasm_prepare
            #block
        }
        
        #native_attr
        #(#attrs)*
        #vis #native_sig {
            #block
        }
    )
    .into()
}

