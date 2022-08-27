//! Contains tools and utils to process, handle and manipulate Rust macros

use proc_macro2::Ident;
use syn::{
    Visibility, Generics, Fields, 
    parse::{
        Parse, ParseBuffer
    }, 
    ItemStruct, Attribute, token::{Enum, Comma}, punctuated::Punctuated, Variant, ItemEnum
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

pub mod processors {
    use proc_macro2::Ident;
    use quote::ToTokens;
    use syn::{Fields, Visibility, Type, Attribute};

    /// TODO Code the get_enum_variants (filter_variants or whatever)
    /// 
    /// TODO Refactor them into a real helper struct
    /// 
    /// Helper for destructure de [`syn::Fields`] into a [`Vec`] of tuples
    /// that holds the attributes of every field.
    pub fn filter_fields(fields: &Fields) -> Vec<(Visibility, Ident, Type, Vec<Attribute>)> {
        fields
            .iter()
            .map( |field| 
                (
                    field.vis.clone(), 
                    field.ident.as_ref().unwrap().clone(),
                    field.ty.clone(),
                    field.attrs.clone()
                ) 
            )
            .collect::<Vec<_>>()
    }



    /// TODO Refactor to a utilery module
    pub fn get_field_type_as_string(typ: &Type) -> String {
        match &*typ {
            Type::Array(type_) => type_.to_token_stream().to_string(),
            Type::BareFn(type_) => type_.to_token_stream().to_string(),
            Type::Group(type_) => type_.to_token_stream().to_string(),
            Type::ImplTrait(type_) => type_.to_token_stream().to_string(),
            Type::Infer(type_) => type_.to_token_stream().to_string(),
            Type::Macro(type_) => type_.to_token_stream().to_string(),
            Type::Never(type_) => type_.to_token_stream().to_string(),
            Type::Paren(type_) => type_.to_token_stream().to_string(),
            Type::Path(type_) => type_.to_token_stream().to_string(),
            Type::Ptr(type_) => type_.to_token_stream().to_string(),
            Type::Reference(type_) => type_.to_token_stream().to_string(),
            Type::Slice(type_) => type_.to_token_stream().to_string(),
            Type::TraitObject(type_) => type_.to_token_stream().to_string(),
            Type::Tuple(type_) => type_.to_token_stream().to_string(),
            Type::Verbatim(type_) => type_.to_token_stream().to_string(),
            _ => "".to_owned(),
        }
    }
}