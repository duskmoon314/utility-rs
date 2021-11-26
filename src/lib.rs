//! # utility_types
//!
//! This crate use proc-macro to realize several utility types of TypeScript
//!
//! | macro            | TypeScript Utility Type                                                                                                     |
//! | ---------------- | --------------------------------------------------------------------------------------------------------------------------- |
//! | [macro@partial]  | [Partial\<Type\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#partialtype)                              |
//! | [macro@required] | [Required\<Type\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#requiredtype)                            |
//! | [macro@pick]     | [Pick\<Type, Keys\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#picktype-keys)                         |
//! | [macro@omit]     | [Omit\<Type, Keys\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#omittype-keys)                         |
//! | [macro@exclude]  | [Exclude\<Type, ExcludedUnion\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#excludetype-excludedunion) |
//! | [macro@extract]  | [Extract\<Type, Union\>](https://www.typescriptlang.org/docs/handbook/utility-types.html#extracttype-union)                 |
//!

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

struct IdentArray {
    _bracket_token: token::Bracket,
    idents: Punctuated<Ident, Token![,]>,
}

impl Parse for IdentArray {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(IdentArray {
            _bracket_token: bracketed!(content in input),
            idents: content.parse_terminated(Ident::parse)?,
        })
    }
}

enum Attribute {
    NoDerives {
        ident: Ident,
        _comma: token::Comma,
        key_idents: IdentArray,
    },
    Derives {
        ident: Ident,
        _comma_1: token::Comma,
        key_idents: IdentArray,
        _comma_2: token::Comma,
        derive_idents: IdentArray,
    },
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let comma: token::Comma = input.parse()?;
        let idents: IdentArray = input.parse()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(token::Comma) {
            Ok(Attribute::Derives {
                ident,
                _comma_1: comma,
                key_idents: idents,
                _comma_2: input.parse()?,
                derive_idents: input.parse()?,
            })
        } else {
            Ok(Attribute::NoDerives {
                ident,
                _comma: comma,
                key_idents: idents,
            })
        }
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

/// Constructs a struct with all properties of the original struct set to optional.
///
/// ### Example
///
/// ```no_run
/// # use utility_types::partial;
/// #[partial(PartialArticle)]
/// struct Article<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: Option<String>
/// }
/// ```
///
/// The code above will become this:
///
/// ```no_run
/// struct Article<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: Option<String>
/// }
///
/// struct PartialArticle<T> {
///     author: Option<String>,
///     content: Option<String>,
///     liked: Option<usize>,
///     comments: Option<T>,
///     link: Option<Option<String>>
/// }
/// ```
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

/// Constructs a struct consisting of all properties of the original struct set to required. The opposite of `partial`.
///
/// ### Example
///
/// ```no_run
/// # use utility_types::required;
/// #[required(RequiredArticle)]
/// struct Article<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: Option<String>
/// }
/// ```
///
/// The code above will become this:
///
/// ```no_run
/// struct Article<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: Option<String>
/// }
///
/// struct RequiredArticle<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: String
/// }
/// ```
///
/// ### Notice
///
/// Currently, only one level of `Option` is removed.
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

/// Constructs a struct by picking the set of fields from the original struct.
///
/// ### Example
///
/// ```no_run
/// # use utility_types::pick;
/// #[pick(ContentComments, [content, comments], [Debug])]
/// struct Article<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: Option<String>
/// }
/// ```
///
/// The code above will become this:
///
/// ```no_run
/// struct Article<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: Option<String>
/// }
///
/// #[derive(Debug)]
/// struct ContentComments<T> {
///     content: String,
///     comments: T,
/// }
/// ```
///
/// ### Notice
///
/// Currently, generics are not analyzed. So rustc will complain if the field with generic is not included in the generated struct.
#[proc_macro_attribute]
pub fn pick(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as Attribute);
    let input = parse_macro_input!(input as ItemStruct);

    let tokens = match attr_input {
        Attribute::NoDerives {
            ident, key_idents, ..
        } => match &input {
            ItemStruct {
                vis,
                generics,
                fields: Fields::Named(fields),
                ..
            } => {
                let fields = fields.named.iter().filter(|f| {
                    key_idents
                        .idents
                        .iter()
                        .any(|k| *k == *f.ident.as_ref().unwrap())
                });
                quote! {
                    #input
                    #vis struct #ident #generics {
                        #(#fields),*
                    }
                }
            }
            _ => Error::new_spanned(&input, "Must define on a struct with named field")
                .to_compile_error(),
        },
        Attribute::Derives {
            ident,
            key_idents,
            derive_idents,
            ..
        } => match &input {
            ItemStruct {
                vis,
                generics,
                fields: Fields::Named(fields),
                ..
            } => {
                let fields = fields.named.iter().filter(|f| {
                    key_idents
                        .idents
                        .iter()
                        .any(|k| *k == *f.ident.as_ref().unwrap())
                });
                let derive_idents = derive_idents.idents;
                quote! {
                    #input
                    #[derive(#derive_idents)]
                    #vis struct #ident #generics {
                        #(#fields),*
                    }
                }
            }
            _ => Error::new_spanned(&input, "Must define on a struct with named field")
                .to_compile_error(),
        },
    };
    tokens.into()
}

/// Constructs a struct by picking fields which not in the set from the original struct.
///
/// ### Example
///
/// ```no_run
/// # use utility_types::omit;
/// #[omit(AuthorLikedComments, [content, link], [Debug])]
/// struct Article<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: Option<String>
/// }
/// ```
///
/// The code above will become this:
///
/// ```no_run
/// struct Article<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: Option<String>
/// }
///
/// #[derive(Debug)]
/// struct AuthorLikedComments<T> {
///     author: String,
///     liked: usize,
///     comments: T,
/// }
/// ```
///
/// ### Notice
///
/// Currently, generics are not analyzed. So rustc will complain if the field with generic is not included in the generated struct.
#[proc_macro_attribute]
pub fn omit(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as Attribute);
    let input = parse_macro_input!(input as ItemStruct);

    let tokens = match attr_input {
        Attribute::NoDerives {
            ident, key_idents, ..
        } => match &input {
            ItemStruct {
                vis,
                generics,
                fields: Fields::Named(fields),
                ..
            } => {
                let fields = fields.named.iter().filter(|f| {
                    key_idents
                        .idents
                        .iter()
                        .all(|k| *k != *f.ident.as_ref().unwrap())
                });
                quote! {
                    #input
                    #vis struct #ident #generics {
                        #(#fields),*
                    }
                }
            }
            _ => Error::new_spanned(&input, "Must define on a struct with named field")
                .to_compile_error(),
        },
        Attribute::Derives {
            ident,
            key_idents,
            derive_idents,
            ..
        } => match &input {
            ItemStruct {
                vis,
                generics,
                fields: Fields::Named(fields),
                ..
            } => {
                let fields = fields.named.iter().filter(|f| {
                    key_idents
                        .idents
                        .iter()
                        .all(|k| *k != *f.ident.as_ref().unwrap())
                });
                let derive_idents = derive_idents.idents;
                quote! {
                    #input
                    #[derive(#derive_idents)]
                    #vis struct #ident #generics {
                        #(#fields),*
                    }
                }
            }
            _ => Error::new_spanned(&input, "Must define on a struct with named field")
                .to_compile_error(),
        },
    };
    tokens.into()
}

/// Constructs an enum by excluding variants in the set from the original enum.
///
/// ### Example
///
/// ```no_run
/// # use utility_types::exclude;
/// #[exclude(Terra, [Jupiter, Saturn, Uranus, Neptune], [Debug])]
/// enum Planet<T> {
///     Mercury(T),
///     Venus(T),
///     Earth(T),
///     Mars(T),
///     Jupiter(T),
///     Saturn(T),
///     Uranus(T),
///     Neptune(T),
/// }
/// ```
///
/// The code above will become this:
///
/// ```no_run
/// enum Planet<T> {
///     Mercury(T),
///     Venus(T),
///     Earth(T),
///     Mars(T),
///     Jupiter(T),
///     Saturn(T),
///     Uranus(T),
///     Neptune(T),
/// }
///
/// #[derive(Debug)]
/// enum Terra<T> {
///     Mercury(T),
///     Venus(T),
///     Earth(T),
///     Mars(T),
/// }
/// ```
///
/// ### Notice
///
/// Currently, generics are not analyzed. So rustc will complain if the field with generic is not included in the generated enum.
#[proc_macro_attribute]
pub fn exclude(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as Attribute);
    let input = parse_macro_input!(input as ItemEnum);

    let tokens = match attr_input {
        Attribute::NoDerives {
            ident, key_idents, ..
        } => {
            let ItemEnum {
                vis,
                generics,
                variants,
                ..
            } = &input;
            let variants = variants
                .iter()
                .filter(|v| key_idents.idents.iter().all(|k| *k != v.ident));
            quote! {
                #input
                #vis enum #ident #generics {
                    #(#variants),*
                }
            }
        }

        Attribute::Derives {
            ident,
            key_idents,
            derive_idents,
            ..
        } => {
            let ItemEnum {
                vis,
                generics,
                variants,
                ..
            } = &input;
            let variants = variants
                .iter()
                .filter(|v| key_idents.idents.iter().all(|k| *k != v.ident));
            let derive_idents = derive_idents.idents;
            quote! {
                #input
                #[derive(#derive_idents)]
                #vis enum #ident #generics {
                    #(#variants),*
                }
            }
        }
    };
    tokens.into()
}

/// Constructs an enum by extracting variants in the set from the original enum.
///
/// ### Example
///
/// ```no_run
/// # use utility_types::extract;
/// #[extract(Terra, [Mercury, Venus, Earth, Mars], [Debug])]
/// enum Planet<T> {
///     Mercury(T),
///     Venus(T),
///     Earth(T),
///     Mars(T),
///     Jupiter(T),
///     Saturn(T),
///     Uranus(T),
///     Neptune(T),
/// }
/// ```
///
/// The code above will become this:
///
/// ```no_run
/// enum Planet<T> {
///     Mercury(T),
///     Venus(T),
///     Earth(T),
///     Mars(T),
///     Jupiter(T),
///     Saturn(T),
///     Uranus(T),
///     Neptune(T),
/// }
///
/// #[derive(Debug)]
/// enum Terra<T> {
///     Mercury(T),
///     Venus(T),
///     Earth(T),
///     Mars(T),
/// }
/// ```
///
/// ### Notice
///
/// Currently, generics are not analyzed. So rustc will complain if the field with generic is not included in the generated enum.
#[proc_macro_attribute]
pub fn extract(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as Attribute);
    let input = parse_macro_input!(input as ItemEnum);

    let tokens = match attr_input {
        Attribute::NoDerives {
            ident, key_idents, ..
        } => {
            let ItemEnum {
                vis,
                generics,
                variants,
                ..
            } = &input;
            let variants = variants
                .iter()
                .filter(|v| key_idents.idents.iter().any(|k| *k == v.ident));
            quote! {
                #input
                #vis enum #ident #generics {
                    #(#variants),*
                }
            }
        }

        Attribute::Derives {
            ident,
            key_idents,
            derive_idents,
            ..
        } => {
            let ItemEnum {
                vis,
                generics,
                variants,
                ..
            } = &input;
            let variants = variants
                .iter()
                .filter(|v| key_idents.idents.iter().any(|k| *k == v.ident));
            let derive_idents = derive_idents.idents;
            quote! {
                #input
                #[derive(#derive_idents)]
                #vis enum #ident #generics {
                    #(#variants),*
                }
            }
        }
    };
    tokens.into()
}
