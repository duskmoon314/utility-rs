use darling::ast::Data;
use darling::util::{Ignored, PathList};
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, Attribute, GenericArgument, Generics, Ident, PathArguments,
    Type, TypePath, Visibility,
};

#[derive(Debug, FromMeta)]
struct RequiredArgs {
    ident: Ident,

    derive: Option<PathList>,
}

#[derive(Debug, FromField)]
#[darling(attributes(required), forward_attrs(allow, doc, cfg))]
struct RequiredField {
    ident: Option<Ident>,

    vis: Visibility,

    ty: Type,

    attrs: Vec<Attribute>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(required),
    forward_attrs(allow, doc, cfg),
    supports(struct_named, struct_tuple)
)]
struct RequiredInput {
    ident: Ident,

    vis: Visibility,

    generics: Generics,

    data: Data<Ignored, RequiredField>,

    #[darling(flatten)]
    args: RequiredArgs,
}

pub fn required(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let input = match RequiredInput::from_derive_input(&input) {
        Ok(input) => input,
        Err(err) => {
            return TokenStream::from(err.write_errors());
        }
    };

    let derive_attr = match input.args.derive {
        Some(derives) => {
            let derives = derives.iter();
            quote! {
                #[derive(#(#derives),*)]
            }
        }
        None => quote! {},
    };
    let vis = input.vis;
    let _ident = input.ident;
    let required_ident = input.args.ident;
    let generics = input.generics;
    let fields = input.data.take_struct().unwrap();

    let mut field_idents = Vec::new();
    let mut field_declares = Vec::new();

    fields.fields.iter().for_each(|field| {
        let vis = &field.vis;
        let ident = field.ident.as_ref().unwrap();
        let attrs = &field.attrs;
        let ty = &field.ty;

        // Check if the field is optional
        let ty = match ty {
            Type::Path(TypePath { path, .. }) => {
                let segments_str = &path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                let option_segment = ["Option", "std::option::Option", "core::option::Option"]
                    .iter()
                    .find(|s| segments_str == *s)
                    .and_then(|_| path.segments.last());
                let inner_type = option_segment
                    .and_then(|path_seg| match &path_seg.arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            args,
                            ..
                        }) => args.first(),
                        _ => None,
                    })
                    .and_then(|generic_arg| match generic_arg {
                        GenericArgument::Type(ty) => Some(ty),
                        _ => None,
                    });

                inner_type.unwrap_or(ty)
            }
            _ => ty,
        };

        field_idents.push(ident.clone());
        field_declares.push(quote! {
            #(#attrs)*
            #vis #ident: #ty
        });
    });

    // TODO: Implement From<RequiredStruct> for OriginalStruct
    quote! {
        #derive_attr
        #vis struct #required_ident #generics {
            #(#field_declares),*
        }
    }
    .into()
}
