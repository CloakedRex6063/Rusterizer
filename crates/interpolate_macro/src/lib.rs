// interpolate_derive/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Interpolate)]
pub fn derive_interpolate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Interpolate only supports named fields"),
        },
        _ => panic!("Interpolate only supports structs"),
    };

    let interpolated_fields = fields.iter().map(|f| {
        let name = f.ident.as_ref().unwrap();
        quote! {
            #name: Interpolate::interp(l0, l1, l2, &a.#name, &b.#name, &c.#name)
        }
    });

    let expanded = quote! {
        impl Interpolate for #name {
            fn interp(
                l0: f32,
                l1: f32,
                l2: f32,
                a: &Self,
                b: &Self,
                c: &Self,
            ) -> Self {
                Self {
                    #(#interpolated_fields,)*
                }
            }
        }
    };

    expanded.into()
}