use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
struct AutoDerefInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<(), AutoDerefFieldsInfo>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>,
}

#[derive(Debug, FromField)]
struct AutoDerefFieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub(crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {
    let AutoDerefInfo {
        ident,
        generics,
        data: darling::ast::Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDeref only works on structs");
    };

    let (fd, ty) = if let Some(field) = field {
        match fields.iter().find(|f| f.ident.as_ref().unwrap() == &field) {
            Some(f) => (field, &f.ty),
            None => panic!("field {field:?} not found in the data structure"),
        }
    } else {
        // if only 1 field, use that field
        if fields.len() == 1 {
            let f = fields.iter().next().unwrap();
            (f.ident.as_ref().unwrap().clone(), &f.ty)
        } else {
            panic!("AutoDeref only works on structs with 1 field or with field attribute");
        }
    };

    let mut code = vec![quote! {
        impl #generics std::ops::Deref for #ident #generics {
            type Target = #ty;

            fn deref(&self) -> &Self::Target {
                &self.#fd
            }
        }
    }];

    if mutable {
        code.push(quote! {
                        impl #generics std::ops::DerefMut for #ident #generics {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#fd
            }
        }
        });
    }

    quote! {
        #(#code)*
    }
}
