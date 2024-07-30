use darling::ast::{Data, Fields, Style};
use darling::util::{Ignored, PathList};
use darling::{FromDeriveInput, FromMeta, FromVariant};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Field, Generics, Ident, Visibility};

use crate::utils::{filter_forward_attrs, ForwardAttrsFilter, IdentList};

#[derive(Debug, FromMeta)]
struct ExcludeArgs {
    ident: Ident,

    variants: IdentList,

    derive: Option<PathList>,

    #[darling(default)]
    forward_attrs: ForwardAttrsFilter,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(exclude), forward_attrs)]
struct ExcludeVariant {
    ident: Ident,

    discriminant: Option<syn::Expr>,

    fields: Fields<Field>,

    attrs: Vec<Attribute>,

    #[darling(default)]
    forward_attrs: ForwardAttrsFilter,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(exclude), forward_attrs, supports(enum_any))]
struct ExcludeInput {
    ident: Ident,

    vis: Visibility,

    generics: Generics,

    data: Data<ExcludeVariant, Ignored>,

    attrs: Vec<Attribute>,

    /// The filter for attributes to forward to the generated enum.
    #[darling(default)]
    forward_attrs: ForwardAttrsFilter,

    /// Args for each generated enum.
    #[darling(multiple, rename = "arg")]
    args: Vec<ExcludeArgs>,
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
        let derive_attr = arg.derive.as_ref().map(|derives| {
            let derives = derives.iter();
            quote! {
                #[derive(#(#derives),*)]
            }
        });

        let forward_attrs = filter_forward_attrs(
            input.attrs.iter(),
            &arg.forward_attrs + &input.forward_attrs,
        );

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

            let forward_attrs = filter_forward_attrs(
                variant.attrs.iter(),
                &variant.forward_attrs + &arg.forward_attrs + &input.forward_attrs,
            );

            let fields = &variant.fields;
            let discriminant = &variant.discriminant;

            variant_idents.push(variant_ident);
            variant_declares.push(quote! {
                #(#forward_attrs)*
                #variant_ident #fields #discriminant
            });
            variant_from_exclude.push(match fields.style {
                Style::Unit => quote! {
                    #exclude_ident::#variant_ident => #ident::#variant_ident
                },
                Style::Tuple => {
                    let field_idents = (0..fields.len())
                        .map(|i| format_ident!("F{i}"))
                        .collect::<Vec<_>>();
                    quote! {
                        #exclude_ident::#variant_ident(#(#field_idents),*) => #ident::#variant_ident(#(#field_idents),*)
                    }
                },
                Style::Struct => {
                    let field_idents = fields
                        .iter()
                        .map(|field| field.ident.as_ref().unwrap())
                        .collect::<Vec<_>>();

                    quote! {
                        #exclude_ident::#variant_ident { #(#field_idents),* } => #ident::#variant_ident { #(#field_idents),* }
                    }
                }
            });
        });

        // TODO: Generics may not be needed in the generated struct
        // It may be better to check all fields for generics
        quote! {
            #derive_attr
            #(#forward_attrs)*
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
