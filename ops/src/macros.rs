//! Contains tools and utils to process, handle and manipulate Rust macros

use proc_macro2::Ident;
use syn::{
    Visibility, Generics, Fields, 
    parse::{
        Parse, ParseBuffer
    }, 
    ItemStruct, Attribute, token::{Enum, Comma}, punctuated::Punctuated, Variant, ItemEnum, DeriveInput
};

/// Parses the tokens of a Rust structure
/// 
/// ## Usage
/// ```
/// let entity: Result<StructParser, Error> = syn::parse::<StructParser>(input);
/// 
///if entity_res.is_err() {
///    return entity_res.err().unwrap().into_compile_error().into()
///}
/// 
/// // No errors detected on the parsing, so we can safely unwrap the parsed result
/// let entity = entity_res.ok().unwrap();
/// ```
#[derive(Clone)]
pub struct StructParser {
    pub ident: Ident,
    pub vis: Visibility,
    pub generics: Generics,
    pub fields: Fields,
    pub attributes: Vec<Attribute>
}

impl Parse for StructParser {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let _struct = input.parse::<ItemStruct>()?;

        Ok(
            Self {
                ident: _struct.ident,
                vis: _struct.vis,
                generics: _struct.generics,
                fields: _struct.fields,
                attributes: _struct.attrs
            }
        )
    }
}


/// Parses the tokens of a Rust enum variant
pub struct EnumParser {
    pub r#enum: Enum,
    pub variants: Punctuated<Variant, Comma>
}

impl Parse for EnumParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _enum = input.parse::<ItemEnum>()?;

        Ok(
            Self {
                r#enum: _enum.enum_token,
                variants: _enum.variants
            }
        )
    }
}