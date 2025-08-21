// proc macro crate

use darling::{FromDeriveInput, FromField, FromVariant};
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

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<EnumVariants, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumVariants {
    ident: syn::Ident,
    fields: darling::ast::Fields<EnumVariantsFields>,
}

#[derive(Debug, FromField)]
struct EnumVariantsFields {
    ty: syn::Type,
}

#[proc_macro_derive(EnumFromDarling)]
pub fn derive_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let EnumFromDarling {
        ident,
        generics,
        data: darling::ast::Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).expect("can not parse input")
    else {
        panic!("EnumFromDarling only works on enums");
    };

    let from_impls = data.iter().map(|variant| {
        let var = &variant.ident;
        let style = &variant.fields.style;
        match style {
            darling::ast::Style::Tuple if variant.fields.len() == 1 => {
                let field = variant.fields.iter().next().expect("should have 1 field");
                let ty = &field.ty;
                quote! {
                    impl #generics From<#ty> for #ident #generics {
                        fn from(v: #ty) -> Self {
                            #ident::#var(v)
                        }
                    }
                }
            }
            _ => quote! {},
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
