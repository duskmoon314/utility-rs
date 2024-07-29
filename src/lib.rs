#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

/// [Exclude] macro implementation.
mod exclude;
/// [Extract] macro implementation.
mod extract;
/// [Omit] macro implementation.
mod omit;
/// [Partial] macro implementation.
mod partial;
/// [Pick] macro implementation.
mod pick;
/// [Required] macro implementation.
mod required;

/// Utility functions and types.
mod utils;

/// Constructs a struct with all fields of the original struct set to optional.
///
/// ## Example
///
/// ```
/// # use utility_types::Partial;
/// #[derive(Partial)]
/// #[partial(ident = PartialArticle, derive(Debug))]
/// pub struct Article {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: String,
/// }
/// ```
///
/// The above code will generate the following struct:
///
/// ```no_run
/// #[derive(Debug)]
/// pub struct PartialArticle {
///     author: Option<String>,
///     content: Option<String>,
///     liked: Option<usize>,
///     comments: Option<String>,
/// }
/// ```
///
/// Several trait implementations are also generated:
/// - `From<Article>` for `PartialArticle`
/// - `From<PartialArticle>` for `Article`
///
/// ## Attributes
///
/// ```ignore
/// #[derive(Partial)]
/// #[partial(
///     ident = <IDENT>, // The identifier of the generated struct
///     [derive(<DERIVE>, ...)], // Derive attributes for the generated struct
///     [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated struct
///         // If no attributes are specified, default attributes are forwarded
///         // `allow`, `cfg`, and `doc`
/// )]
/// pub struct BasedStruct {
///     #[partial(
///         [default = <DEFAULT>], // The default value of the field in the generated From impl
///         [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated field
///             // If given, will override the container level `forward_attrs`
///     )]
///     field: FieldType,
/// }
/// ```
#[proc_macro_derive(Partial, attributes(partial))]
pub fn partial(input: TokenStream) -> TokenStream {
    partial::partial(input)
}

/// Constructs a struct with all fields of the original struct set to required.
///
/// ## Example
///
/// ```
/// # use utility_types::Required;
/// #[derive(Required)]
/// #[required(ident = RequiredArticle, derive(Debug))]
/// pub struct Article {
///     author: String,
///     content: Option<String>,
/// }
/// ```
///
/// The above code will generate the following struct:
///
/// ```
/// #[derive(Debug)]
/// pub struct RequiredArticle {
///     author: String,
///     content: String,
/// }
/// ```
///
/// ## Attributes
///
/// ```ignore
/// #[derive(Required)]
/// #[required(
///     ident = <IDENT>, // The identifier of the generated struct
///     [derive(<DERIVE>, ...)], // Derive attributes for the generated struct
///     [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated struct
///         // If no attributes are specified, default attributes are forwarded
///         // `allow`, `cfg`, and `doc`
/// )]
/// pub struct BasedStruct {
///     #[required(
///         [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated field
///             // If given, will override the container level `forward_attrs`
///     )]
///     field: FieldType,
/// }
/// ```
#[proc_macro_derive(Required, attributes(required))]
pub fn required(input: TokenStream) -> TokenStream {
    required::required(input)
}

/// Constructs a struct by picking the set of fields from the original struct.
///
/// ## Example
///
/// ```ignore
/// # use utility_types::Pick;
/// #[derive(Pick)]
/// #[pick(arg(ident = AuthorContent, fields(author, content), derive(Debug)))]
/// #[pick(arg(ident = LikedComments, fields(liked, comments)))]
/// pub struct Article {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: String,
/// }
/// ```
///
/// The above code will generate the following structs:
///
/// ```
/// #[derive(Debug)]
/// pub struct AuthorContent {
///     author: String,
///     content: String,
/// }
///
/// pub struct LikedComments {
///     liked: usize,
///     comments: String,
/// }
/// ```
///
/// Several trait implementations are also generated:
/// - `From<Article>` for `AuthorContent`
/// - `From<Article>` for `LikedComments`
///
/// ## Attributes
///
/// ```ignore
/// # use utility_types::Pick;
/// #[derive(Pick)]
/// #[pick(
///     [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated struct
///         // If no attributes are specified, default attributes are forwarded
///         // `allow`, `cfg`, and `doc`
/// )]
/// #[pick(
///     arg(
///         ident = <IDENT>, // The identifier of the generated struct
///         fields(<FIELD>, ...), // The fields to pick from the original struct
///         [derive(<DERIVE>, ...)], // Derive attributes for the generated struct
///         [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated struct
///             // If given, will override the container level `forward_attrs`
///     ),
/// )]
/// pub struct BasedStruct {
///     #[pick(
///         #[forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated field
///             // If given, will override the container level and arg level `forward_attrs`
///     )]
///     field: FieldType,
/// }
#[proc_macro_derive(Pick, attributes(pick))]
pub fn pick(input: TokenStream) -> TokenStream {
    pick::pick(input)
}

/// Constructs a struct by omitting the set of fields from the original struct.
///
/// ## Example
///
/// ```
/// # use utility_types::Omit;
/// #[derive(Omit)]
/// #[omit(arg(ident = OmitAuthorContent, fields(author, content), derive(Debug)))]
/// #[omit(arg(ident = OmitLikedComments, fields(liked, comments)))]
/// pub struct Article {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: String,
/// }
/// ```
///
/// The above code will generate the following structs:
///
/// ```
/// #[derive(Debug)]
/// pub struct OmitAuthorContent {
///     liked: usize,
///     comments: String,
/// }
///
/// pub struct OmitLikedComments {
///     author: String,
///     content: String,
/// }
/// ```
///
/// Several trait implementations are also generated:
///
/// - `From<Article>` for `OmitAuthorContent`
/// - `From<Article>` for `OmitLikedComments`
///
/// ## Attributes
///
/// ```ignore
/// #[derive(Omit)]
/// #[omit(
///     [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated struct
///         // If no attributes are specified, default attributes are forwarded
///         // `allow`, `cfg`, and `doc`
/// )]
/// #[omit(
///     arg(
///         ident = <IDENT>, // The identifier of the generated struct
///         fields(<FIELD>, ...), // The fields to omit from the original struct
///         [derive(<DERIVE>, ...)], // Derive attributes for the generated struct
///         [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated struct
///             // If given, will override the container level `forward_attrs`
///     ),
/// )]
/// pub struct BasedStruct {
///     #[omit(
///         #[forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated field
///             // If given, will override the container level and arg level `forward_attrs`
///     )]
///     field: FieldType,
/// }
/// ```
#[proc_macro_derive(Omit, attributes(omit))]
pub fn omit(input: TokenStream) -> TokenStream {
    omit::omit(input)
}

/// Constructs an enum by extracting the set of variants from the original enum.
///
/// ## Example
///
/// ```
/// # use utility_types::Extract;
/// #[derive(Extract)]
/// #[extract(arg(ident = ExtractMercury, variants(Mercury)))]
/// pub enum Planet {
///     Mercury,
///     Venus,
///     Earth,
///     Mars,
/// }
/// ```
///
/// The above code will generate the following enum:
///
/// ```
/// pub enum ExtractMercury {
///     Mercury,
/// }
/// ```
///
/// ## Attributes
///
/// ```ignore
/// #[derive(Extract)]
/// #[extract(
///     [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated enum
///         // If no attributes are specified, default attributes are forwarded
///         // `allow`, `cfg`, and `doc`
/// )]
/// #[extract(
///     arg(
///         ident = <IDENT>, // The identifier of the generated enum
///         variants(<VARIANT>, ...), // The variants to extract from the original enum
///         [derive(<DERIVE>, ...)], // Derive attributes for the generated enum
///         [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated enum
///             // If given, will override the container level `forward_attrs`
///     ),
/// )]
/// pub enum BasedEnum {
///     #[extract(
///         [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated variant
///             // If given, will override the container level and arg level `forward_attrs`
///     )]
///     variant: VariantType,
/// }
/// ```
#[proc_macro_derive(Extract, attributes(extract))]
pub fn extract(input: TokenStream) -> TokenStream {
    extract::extract(input)
}

/// Constructs an enum by excluding the set of variants from the original enum.
///
/// ## Example
///
/// ```
/// # use utility_types::Exclude;
/// #[derive(Exclude)]
/// #[exclude(arg(ident = ExcludeMercury, variants(Mercury)))]
/// pub enum Planet {
///     Mercury,
///     Venus,
///     Earth,
///     Mars,
/// }
/// ```
///
/// The above code will generate the following enum:
///
/// ```
/// pub enum ExcludeMercury {
///     Venus,
///     Earth,
///     Mars,
/// }
/// ```
///
/// ## Attributes
///
/// ```ignore
/// #[derive(Exclude)]
/// #[exclude(
///     [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated enum
///         // If no attributes are specified, default attributes are forwarded
///         // `allow`, `cfg`, and `doc`
/// )]
/// #[exclude(
///     arg(
///         ident = <IDENT>, // The identifier of the generated enum
///         variants(<VARIANT>, ...), // The variants to exclude from the original enum
///         [derive(<DERIVE>, ...)], // Derive attributes for the generated enum
///         [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated enum
///             // If given, will override the container level `forward_attrs`
///     ),
/// )]
/// pub enum BasedEnum {
///     #[exclude(
///         [forward_attrs(<ATTR>, ...)], // Forward specific attributes to the generated variant
///             // If given, will override the container level and arg level `forward_attrs`
///     )]
///     variant: VariantType,
/// }
/// ```
#[proc_macro_derive(Exclude, attributes(exclude))]
pub fn exclude(input: TokenStream) -> TokenStream {
    exclude::exclude(input)
}
