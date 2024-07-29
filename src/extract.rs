use std::ops::Deref;

use darling::ast::{Data, Fields, Style};
use darling::util::{Ignored, PathList};
use darling::{FromDeriveInput, FromMeta, FromVariant};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Field, Generics, Ident, Visibility};

use crate::utils::{default_forward_attrs_filter, IdentList};

#[derive(Debug, FromMeta)]
struct ExtractArgs {
    ident: Ident,

    variants: IdentList,

    derive: Option<PathList>,

    forward_attrs: Option<PathList>,
}

#[derive(Debug)]
struct ExtractArgsList(Vec<ExtractArgs>);

impl Deref for ExtractArgsList {
    type Target = Vec<ExtractArgs>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for ExtractArgsList {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        let values = items
            .iter()
            .map(|item| match item {
                darling::ast::NestedMeta::Meta(meta) => ExtractArgs::from_meta(meta),
                _ => Err(darling::Error::unexpected_type("non meta").with_span(item)),
            })
            .collect::<darling::Result<Vec<ExtractArgs>>>()?;

        Ok(Self(values))
    }
}

#[derive(Debug, FromVariant)]
#[darling(attributes(extract), forward_attrs)]
struct ExtractVariant {
    ident: Ident,

    discriminant: Option<syn::Expr>,

    fields: Fields<Field>,

    attrs: Vec<Attribute>,

    forward_attrs: Option<PathList>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(extract), forward_attrs, supports(enum_any))]
struct ExtractInput {
    ident: Ident,

    vis: Visibility,

    generics: Generics,

    data: Data<ExtractVariant, Ignored>,

    attrs: Vec<Attribute>,

    /// The filter for attributes to forward to the generated enum.
    forward_attrs: Option<PathList>,

    #[darling(flatten)]
    args: ExtractArgsList,
}

pub fn extract(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let input = match ExtractInput::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    let vis = input.vis;
    let ident = input.ident;
    let generics = input.generics;
    let variants = input.data.take_enum().unwrap();

    let extracts = input.args.iter().map(|arg| {
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

        let extract_ident = &arg.ident;

        let mut variant_idents = Vec::new();
        let mut variant_declares = Vec::new();
        let mut variant_from_extract = Vec::new();

        variants.iter().for_each(|variant| {
            let variant_ident = &variant.ident;

            // Check if ident is in the list of variants to extract
            if !arg.variants.contains(variant_ident) {
                return;
            }

            let forward_attrs = variant
                .forward_attrs
                .as_ref()
                .or(arg.forward_attrs.as_ref())
                .or(input.forward_attrs.as_ref());
            let forward_attrs = variant.attrs.iter().filter(|attr| match forward_attrs {
                Some(filter) => filter.contains(attr.path()),
                None => default_forward_attrs_filter(attr.path()),
            });

            let fields = &variant.fields;
            let discriminant = &variant.discriminant;

            variant_idents.push(variant_ident);
            variant_declares.push(quote! {
                #(#forward_attrs)*
                #variant_ident #fields #discriminant
            });
            variant_from_extract.push(match fields.style {
                Style::Unit => quote! {
                    #extract_ident::#variant_ident => #ident::#variant_ident
                },
                Style::Tuple => {
                    let field_idents = (0..fields.len())
                        .map(|i| format_ident!("F{i}"))
                        .collect::<Vec<_>>();

                    quote! {
                        #extract_ident::#variant_ident(#(#field_idents),*) => #ident::#variant_ident(#(#field_idents),*)
                    }
                },
                Style::Struct => {
                    let field_idents = fields
                        .iter()
                        .map(|field| field.ident.as_ref().unwrap())
                        .collect::<Vec<_>>();

                    quote! {
                        #extract_ident::#variant_ident { #(#field_idents),* } => #ident::#variant_ident { #(#field_idents),* }
                    }
                }
            });
        });

        // TODO: Generics may not be needed in the generated struct
        // It may be better to check all fields for generics
        quote! {
            #derive_attr
            #(#forward_attrs)*
            #vis enum #extract_ident #generics {
                #(#variant_declares),*
            }

            impl #generics core::convert::From<#extract_ident #generics> for #ident #generics {
                fn from(src: #extract_ident #generics) -> Self {
                    match src {
                        #(#variant_from_extract),*
                    }
                }
            }
        }
    });

    quote! {
        #(#extracts)*
    }
    .into()
}
