// proc macro crate

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    println!("{input:#?}");

    let ident = input.ident;
    let generic = input.generics;

    let data = input.data;

    let variants = match data {
        syn::Data::Enum(data_enum) => data_enum.variants,
        _ => panic!("EnumFrom only works on enums"),
    };

    let from_impls = variants.iter().map(|v| {
        let var = &v.ident;
        match &v.fields {
            syn::Fields::Named(_fields_named) => quote! {},
            syn::Fields::Unnamed(fields_unnamed) => {
                if fields_unnamed.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let filed = fields_unnamed.unnamed.first().expect("should have 1 field");
                    let ty = &filed.ty;
                    quote! {
                        impl #generic From<#ty> for #ident #generic {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        assert_eq!(1, 1);
    }
}
