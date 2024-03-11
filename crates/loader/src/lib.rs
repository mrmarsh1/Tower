use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

extern crate proc_macro;

#[proc_macro_attribute]
pub fn tower_app(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let new_fn = quote! {
        use wasm_bindgen::prelude::*;

        fn #fn_name() -> Result<(), JsValue> {
        #[wasm_bindgen(start)]

            #input_fn
            Ok(())
        }
    };

    new_fn.into()
}
