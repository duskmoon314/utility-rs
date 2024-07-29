use std::ops::Deref;

use darling::ast::Data;
use darling::util::{Ignored, PathList};
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Generics, Ident, Type, Visibility};

use crate::utils::{default_forward_attrs_filter, IdentList};

#[derive(Debug, FromMeta)]
struct OmitArgs {
    ident: Ident,

    fields: IdentList,

    derive: Option<PathList>,

    forward_attrs: Option<PathList>,
}

#[derive(Debug)]
struct OmitArgsList(Vec<OmitArgs>);

impl Deref for OmitArgsList {
    type Target = Vec<OmitArgs>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for OmitArgsList {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        let values = items
            .iter()
            .map(|item| match item {
                darling::ast::NestedMeta::Meta(meta) => OmitArgs::from_meta(meta),
                _ => Err(darling::Error::unexpected_type("non meta").with_span(item)),
            })
            .collect::<darling::Result<Vec<OmitArgs>>>()?;

        Ok(Self(values))
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(omit), forward_attrs)]
struct OmitField {
    ident: Option<Ident>,

    vis: Visibility,

    ty: Type,

    attrs: Vec<Attribute>,

    forward_attrs: Option<PathList>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(omit), forward_attrs, supports(struct_named))]
struct OmitInput {
    ident: Ident,

    vis: Visibility,

    generics: Generics,

    data: Data<Ignored, OmitField>,

    attrs: Vec<Attribute>,

    /// The filter for attributes to forward to the generated struct
    forward_attrs: Option<PathList>,

    /// Args for each generated struct
    #[darling(flatten)]
    args: OmitArgsList,
}

pub fn omit(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let input = match OmitInput::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    let vis = input.vis;
    let ident = input.ident;
    let generics = input.generics;
    let fields = input.data.take_struct().unwrap();

    let omits = input.args.iter().map(|arg| {
        let derive_attr = arg.derive.as_ref().map(|derives| {
            let derives = derives.iter();
            quote! {
                #[derive(#(#derives),*)]
            }
        });

        let forward_attrs = arg.forward_attrs.as_ref().or(input.forward_attrs.as_ref());
        let forward_attrs = input.attrs.iter().filter(|attr| match forward_attrs {
            Some(filter) => filter.contains(attr.path()),
            None => default_forward_attrs_filter(attr.path()),
        });

        let omit_ident = &arg.ident;

        let mut field_idents = Vec::new();
        let mut field_declares = Vec::new();

        fields.fields.iter().for_each(|field| {
            let ident = field.ident.as_ref().unwrap();

            // Check if ident is in the list of fields to Omit
            if arg.fields.contains(ident) {
                return;
            }

            let forward_attrs = field
                .forward_attrs
                .as_ref()
                .or(arg.forward_attrs.as_ref())
                .or(input.forward_attrs.as_ref());
            let forward_attrs = field.attrs.iter().filter(|attr| match forward_attrs {
                Some(filter) => filter.contains(attr.path()),
                None => default_forward_attrs_filter(attr.path()),
            });

            let vis = &field.vis;
            let ty = &field.ty;

            field_idents.push(ident);
            field_declares.push(quote! {
                #(#forward_attrs)*
                #vis #ident: #ty,
            });
        });

        // TODO: Generics may not be needed in the generated struct
        // It may be better to check all fields for generics
        quote! {
            #derive_attr
            #(#forward_attrs)*
            #vis struct #omit_ident #generics {
                #(#field_declares)*
            }

            impl #generics From<#ident #generics> for #omit_ident #generics {
                fn from(src: #ident #generics) -> Self {
                    Self {
                        #(#field_idents: src.#field_idents),*
                    }
                }
            }
        }
    });

    quote! {
        #(#omits)*
    }
    .into()
}
