#![allow(clippy::eval_order_dependence)]

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    bracketed, parse_macro_input, token, Error, Fields, ItemEnum, ItemStruct, Result, Token, Type,
};

mod kw {
    syn::custom_keyword!(Option);
}

struct UtilityAttribute {
    ident: Ident,
    _comma_1: token::Comma,
    _bracket_token_1: token::Bracket,
    field_idents: Punctuated<Ident, Token![,]>,
    _comma_2: token::Comma,
    _bracket_token_2: token::Bracket,
    derive_idents: Punctuated<Ident, Token![,]>,
}

impl Parse for UtilityAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content_1;
        let content_2;
        Ok(UtilityAttribute {
            ident: input.parse()?,
            _comma_1: input.parse()?,
            _bracket_token_1: bracketed!(content_1 in input),
            field_idents: content_1.parse_terminated(Ident::parse)?,
            _comma_2: input.parse()?,
            _bracket_token_2: bracketed!(content_2 in input),
            derive_idents: content_2.parse_terminated(Ident::parse)?,
        })
    }
}

struct TypeOption {
    _option_token: kw::Option,
    _lt_token: Token![<],
    ty: Type,
    _rt_token: Token![>],
}

impl Parse for TypeOption {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(TypeOption {
            _option_token: input.parse()?,
            _lt_token: input.parse()?,
            ty: input.parse()?,
            _rt_token: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn partial(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ident = parse_macro_input!(attr as Ident);
    let input = parse_macro_input!(input as ItemStruct);

    let tokens = match &input {
        ItemStruct {
            vis,
            generics,
            fields: Fields::Named(fields),
            ..
        } => {
            let field_vis = fields.named.iter().map(|f| &f.vis);
            let field_ident = fields.named.iter().map(|f| &f.ident);
            let field_type = fields.named.iter().map(|f| &f.ty);
            quote! {
                #input
                #vis struct #attr_ident #generics {
                    #(#field_vis #field_ident: Option<#field_type>),*
                }
            }
        }
        ItemStruct {
            vis,
            generics,
            fields: Fields::Unnamed(fields),
            ..
        } => {
            let field_vis = fields.unnamed.iter().map(|f| &f.vis);
            let field_type = fields.unnamed.iter().map(|f| &f.ty);
            quote! {
                #input
                #vis struct #attr_ident #generics (#(#field_vis Option<#field_type>),*);
            }
        }
        _ => Error::new_spanned(&input, "Must define on a struct with fields").to_compile_error(),
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn required(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_ident = parse_macro_input!(attr as Ident);
    let input = parse_macro_input!(input as ItemStruct);
    let tokens = match &input {
        ItemStruct {
            vis,
            generics,
            fields: Fields::Named(fields),
            ..
        } => {
            let field_vis = fields.named.iter().map(|f| &f.vis);
            let field_ident = fields.named.iter().map(|f| &f.ident);
            let field_type = fields.named.iter().map(|f| match &f.ty {
                Type::Path(type_path) => {
                    let parse_option = syn::parse::<TypeOption>(TokenStream::from(
                        type_path.path.segments.to_token_stream(),
                    ));
                    match parse_option {
                        Ok(option) => option.ty,
                        _ => f.ty.clone(),
                    }
                }
                _ => f.ty.clone(),
            });
            quote! {
                #input
                #vis struct #attr_ident #generics {
                    #(#field_vis #field_ident: #field_type),*
                }
            }
        }
        ItemStruct {
            vis,
            generics,
            fields: Fields::Unnamed(fields),
            ..
        } => {
            let field_vis = fields.unnamed.iter().map(|f| &f.vis);
            let field_type = fields.unnamed.iter().map(|f| &f.ty);
            quote! {
                #input
                #vis struct #attr_ident #generics (#(#field_vis #field_type),*);
            }
        }
        _ => Error::new_spanned(&input, "Must define on a struct with fields").to_compile_error(),
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn pick(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as UtilityAttribute);
    let input = parse_macro_input!(input as ItemStruct);

    let attr_ident = attr_input.ident;
    let attr_fields = attr_input.field_idents;
    let attr_derive = attr_input.derive_idents;

    let tokens = match &input {
        ItemStruct {
            vis,
            generics,
            fields: Fields::Named(fields),
            ..
        } => {
            let fields = fields.named.iter().filter(|v| {
                attr_fields
                    .iter()
                    .any(|av| *av == *v.ident.as_ref().unwrap())
            });
            quote! {
                #input
                #[derive(#attr_derive)]
                #vis struct #attr_ident #generics {
                    #(#fields),*
                }
            }
        }
        _ => Error::new_spanned(&input, "Must define on a struct with named field")
            .to_compile_error(),
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn omit(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as UtilityAttribute);
    let input = parse_macro_input!(input as ItemStruct);

    let attr_ident = attr_input.ident;
    let attr_fields = attr_input.field_idents;
    let attr_derive = attr_input.derive_idents;

    let tokens = match &input {
        ItemStruct {
            vis,
            generics,
            fields: Fields::Named(fields),
            ..
        } => {
            let fields = fields.named.iter().filter(|v| {
                attr_fields
                    .iter()
                    .all(|av| *av != *v.ident.as_ref().unwrap())
            });
            quote! {
                #input
                #[derive(#attr_derive)]
                #vis struct #attr_ident #generics {
                    #(#fields),*
                }
            }
        }
        _ => Error::new_spanned(&input, "Must define on a struct with named field")
            .to_compile_error(),
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn exclude(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as UtilityAttribute);
    let input = parse_macro_input!(input as ItemEnum);

    let attr_ident = attr_input.ident;
    let attr_variants = attr_input.field_idents;
    let attr_derive = attr_input.derive_idents;

    let ItemEnum {
        vis,
        generics,
        variants,
        ..
    } = &input;

    let variants = variants
        .iter()
        .filter(|v| attr_variants.iter().all(|av| *av != v.ident));
    let tokens = quote! {
        #input
        #[derive(#attr_derive)]
        #vis enum #attr_ident #generics {
            #(#variants),*
        }
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn extract(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as UtilityAttribute);
    let input = parse_macro_input!(input as ItemEnum);

    let attr_ident = attr_input.ident;
    let attr_variants = attr_input.field_idents;
    let attr_derive = attr_input.derive_idents;

    let ItemEnum {
        vis,
        generics,
        variants,
        ..
    } = &input;

    let variants = variants
        .iter()
        .filter(|v| attr_variants.iter().any(|av| *av == v.ident));
    let tokens = quote! {
        #input
        #[derive(#attr_derive)]
        #vis enum #attr_ident #generics {
            #(#variants),*
        }
    };

    tokens.into()
}
