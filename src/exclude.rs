use std::ops::Deref;

use darling::ast::{Data, Fields, Style};
use darling::util::{Ignored, PathList};
use darling::{FromDeriveInput, FromMeta, FromVariant};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Field, Generics, Ident, Visibility};

use crate::utils::IdentList;

#[derive(Debug, FromMeta)]
struct ExcludeArgs {
    ident: Ident,

    variants: IdentList,

    derive: Option<PathList>,
}

#[derive(Debug)]
struct ExcludeArgsList(Vec<ExcludeArgs>);

impl Deref for ExcludeArgsList {
    type Target = Vec<ExcludeArgs>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromMeta for ExcludeArgsList {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        let values = items
            .iter()
            .map(|item| match item {
                darling::ast::NestedMeta::Meta(meta) => ExcludeArgs::from_meta(meta),
                _ => Err(darling::Error::unexpected_type("non meta").with_span(item)),
            })
            .collect::<darling::Result<Vec<ExcludeArgs>>>()?;

        Ok(Self(values))
    }
}

#[derive(Debug, FromVariant)]
#[darling(attributes(exclude), forward_attrs(allow, doc, cfg))]
struct ExcludeVariant {
    ident: Ident,

    discriminant: Option<syn::Expr>,

    fields: Fields<Field>,

    attrs: Vec<Attribute>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(exclude),
    forward_attrs(allow, doc, cfg),
    supports(enum_any)
)]
struct ExcludeInput {
    ident: Ident,

    vis: Visibility,

    generics: Generics,

    data: Data<ExcludeVariant, Ignored>,

    #[darling(flatten)]
    args: ExcludeArgsList,
}

pub fn exclude(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let input = match ExcludeInput::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    let vis = input.vis;
    let ident = input.ident;
    let generics = input.generics;
    let variants = input.data.take_enum().unwrap();

    let excludes = input.args.iter().map(|arg| {
        let derive_attr = match &arg.derive {
            Some(derives) => {
                let derives = derives.iter();
                quote! {
                    #[derive(#(#derives),*)]
                }
            }
            None => quote! {},
        };

        let exclude_ident = &arg.ident;

        let mut variant_idents = Vec::new();
        let mut variant_declares = Vec::new();
        let mut variant_from_exclude = Vec::new();

        variants.iter().for_each(|variant| {
            let variant_ident = &variant.ident;

            // Check if ident is in the list of variants to Exclude
            if arg.variants.contains(variant_ident) {
                return;
            }

            let attrs = &variant.attrs;
            let fields = &variant.fields;
            let discriminant = &variant.discriminant;

            variant_idents.push(variant_ident);
            variant_declares.push(quote! {
                #(#attrs)*
                #variant_ident #fields #discriminant
            });
            variant_from_exclude.push(match fields.style {
                Style::Unit => quote! {
                    #exclude_ident::#variant_ident => #ident::#variant_ident
                },
                Style::Tuple => {
                    let field_idents = (0..fields.len()).map(|i| format_ident!("F{i}")).collect::<Vec<_>>();

                    quote! {
                        #exclude_ident::#variant_ident(#(#field_idents),*) => #ident::#variant_ident(#(#field_idents),*)
                    }
                }
                Style::Struct => {
                    let field_idents = fields.iter().map(|field| field.ident.as_ref().unwrap()).collect::<Vec<_>>();

                    quote! {
                        #exclude_ident::#variant_ident { #(#field_idents),* } => #ident::#variant_ident { #(#field_idents),* }
                    }
                }
            })
        });

        // TODO: Generics may not be needed in the generated struct
        // It may be better to check all fields for generics
        quote! {
            #derive_attr
            #vis enum #exclude_ident #generics {
                #(#variant_declares),*
            }

            impl #generics core::convert::From<#exclude_ident #generics> for #ident #generics {
                fn from(src: #exclude_ident #generics) -> Self {
                    match src {
                        #(#variant_from_exclude),*
                    }
                }
            }
        }
    });

    quote! {
        #(#excludes)*
    }
    .into()
}
