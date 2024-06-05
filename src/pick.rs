use std::ops::Deref;

use darling::ast::Data;
use darling::util::{Ignored, PathList};
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Generics, Ident, Type, Visibility};

use crate::utils::IdentList;

#[derive(Debug, FromMeta)]
struct PickArgs {
    ident: Ident,

    fields: IdentList,

    derive: Option<PathList>,
}

#[derive(Debug)]
struct PickArgsList(Vec<PickArgs>);

impl Deref for PickArgsList {
    type Target = Vec<PickArgs>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for PickArgsList {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        let values = items
            .iter()
            .map(|item| match item {
                darling::ast::NestedMeta::Meta(meta) => PickArgs::from_meta(meta),
                _ => Err(darling::Error::unexpected_type("non meta").with_span(item)),
            })
            .collect::<darling::Result<Vec<PickArgs>>>()?;

        Ok(Self(values))
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(pick), forward_attrs(allow, doc, cfg))]
struct PickField {
    ident: Option<Ident>,

    vis: Visibility,

    ty: Type,

    attrs: Vec<Attribute>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(pick),
    forward_attrs(allow, doc, cfg),
    supports(struct_named)
)]
struct PickInput {
    ident: Ident,

    vis: Visibility,

    generics: Generics,

    data: Data<Ignored, PickField>,

    #[darling(flatten)]
    args: PickArgsList,
}

pub fn pick(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let input = match PickInput::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    let vis = input.vis;
    let ident = input.ident;
    let generics = input.generics;
    let fields = input.data.take_struct().unwrap();

    let picks = input.args.iter().map(|arg| {
        let derive_attr = match &arg.derive {
            Some(derives) => {
                let derives = derives.iter();
                quote! {
                    #[derive(#(#derives),*)]
                }
            }
            None => quote! {},
        };

        let pick_ident = &arg.ident;

        let mut field_idents = Vec::new();
        let mut field_declares = Vec::new();

        fields.fields.iter().for_each(|field| {
            let ident = field.ident.as_ref().unwrap();

            // Check if ident is in the list of fields to pick
            if !arg.fields.contains(ident) {
                return;
            }

            let attrs = &field.attrs;
            let vis = &field.vis;
            let ty = &field.ty;

            field_idents.push(ident);
            field_declares.push(quote! {
                #(#attrs)*
                #vis #ident: #ty,
            });
        });

        // TODO: Generics may not be needed in the generated struct
        // It may be better to check all fields for generics
        quote! {
            #derive_attr
            #vis struct #pick_ident #generics {
                #(#field_declares)*
            }

            impl #generics From<#ident #generics> for #pick_ident #generics {
                fn from(src: #ident #generics) -> Self {
                    Self {
                        #(#field_idents: src.#field_idents),*
                    }
                }
            }
        }
    });

    quote! {
        #(#picks)*
    }
    .into()
}
