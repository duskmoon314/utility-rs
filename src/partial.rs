use darling::ast::Data;
use darling::util::{Ignored, PathList};
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Generics, Ident, Type, Visibility};

use crate::utils::{filter_forward_attrs, ForwardAttrsFilter};

#[derive(Debug, FromMeta)]
struct PartialArgs {
    ident: Ident,

    derive: Option<PathList>,

    #[darling(default)]
    forward_attrs: ForwardAttrsFilter,
}

#[derive(Debug, FromField)]
#[darling(attributes(partial), forward_attrs)]
struct PartialField {
    ident: Option<Ident>,

    vis: Visibility,

    ty: Type,

    attrs: Vec<Attribute>,

    default: Option<syn::Expr>,

    #[darling(default)]
    forward_attrs: ForwardAttrsFilter,
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(partial),
    forward_attrs,
    supports(struct_named, struct_tuple)
)]
struct PartialInput {
    ident: Ident,

    vis: Visibility,

    generics: Generics,

    data: Data<Ignored, PartialField>,

    attrs: Vec<Attribute>,

    #[darling(flatten)]
    args: PartialArgs,
}

pub fn partial(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let input = match PartialInput::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    let derive_attr = input.args.derive.as_ref().map(|derives| {
        let derives = derives.iter();
        quote! {
            #[derive(#(#derives),*)]
        }
    });

    let forward_attrs = filter_forward_attrs(input.attrs.iter(), &input.args.forward_attrs);

    let vis = input.vis;
    let ident = input.ident;
    let partial_ident = input.args.ident;
    let generics = input.generics;
    let fields = input.data.take_struct().unwrap();

    let mut field_idents = Vec::new();
    let mut field_declares = Vec::new();
    let mut field_from_partial = Vec::new();

    fields.fields.iter().for_each(|field| {
        let vis = &field.vis;
        let ident = field.ident.as_ref().unwrap();

        let forward_attrs = filter_forward_attrs(
            field.attrs.iter(),
            &field.forward_attrs + &input.args.forward_attrs,
        );

        let ty = &field.ty;
        // TODO: It may be better to keep the original type if it is already an Option
        let ty = quote! { Option<#ty> };

        field_idents.push(ident.clone());
        field_declares.push(quote! {
            #(#forward_attrs)*
            #vis #ident: #ty
        });
        field_from_partial.push(match &field.default {
            Some(default) => quote! {
                #ident: src.#ident.unwrap_or(#default)
            },
            None => quote! {
                #ident: src.#ident.unwrap_or_default()
            },
        });
    });

    quote! {
        #derive_attr
        #(#forward_attrs)*
        #vis struct #partial_ident #generics {
            #(#field_declares),*
        }

        impl #generics core::convert::From<#ident #generics> for #partial_ident #generics {
            fn from(src: #ident #generics) -> Self {
                Self {
                    #(#field_idents: Some(src.#field_idents)),*
                }
            }
        }

        impl #generics core::convert::From<#partial_ident #generics> for #ident #generics {
            fn from(src: #partial_ident #generics) -> Self {
                Self {
                    #(#field_from_partial),*
                }
            }
        }
    }
    .into()
}
