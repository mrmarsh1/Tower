use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn component(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let ident = &item_struct.ident;

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.insert(0,
            syn::Field::parse_named.parse2(quote!{pub entity: usize})
            .unwrap(),
        );
        fields.named.insert(1,
            syn::Field::parse_named.parse2(quote!{pub one_frame: bool})
            .unwrap(),
        );
    }

    return quote!{
        #item_struct

        impl Component for #ident {
            fn get_id() -> usize {
                core::any::TypeId::of::<Self> as usize
            }

            fn entity(&self) -> usize {
                self.entity
            }

            fn one_frame(&self) -> bool {
                self.one_frame
            }
        }
    }.into()
}
