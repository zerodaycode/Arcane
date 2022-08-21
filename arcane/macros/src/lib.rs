//! Library for the implementation of the macro-driven `Arcane` reflective features

extern crate proc_macro;

use arcane_reflexion::ItemVisibility;
use proc_macro::TokenStream as CompilerTokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{
    DeriveInput, Fields, Type, Visibility, Attribute, Generics
};



/// Convenient structure to parse the tokens of a Rust structure
struct StructParser<'a> {
    pub struct_name: &'a str,
    pub vis: ItemVisibility,
    pub generics: Generics,
    pub fields: &'a [Fields],
    pub attributes: &'a [Attribute]
}  /// TODO Como parser, as processor, pero en el módulo de
    // reflexion, como sun helper



#[proc_macro_derive(Reflexion)]
pub fn reflexion_struct_details(input: CompilerTokenStream) -> CompilerTokenStream {
    // Getting data from the AST
    let ast: DeriveInput = syn::parse(input).unwrap();
    let ty = ast.ident;
    let ty_str = ty.to_string();

    let ast_data = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => return syn::Error::new(
            ty.span(), 
            "Reflection only works with structs"
        )
        .to_compile_error()
        .into()
    };

    // Recovers the identifiers of the struct's members, and checks that the derive
    // macro it's only applied to structs
    let fields = filter_fields(ast_data);

    // Generates the tokens for create the relationship between the fields and it's
    // declared types
    let hm_f = fields.iter()
        .map( |(_vis, ident, typ, _attrs)| 
            {
                let i = ident.to_string();
                let t = get_field_type_as_string(typ);
                quote! { hm.insert(#i, #t); }
            }
    );

    // Generates the [`StructInfo`] entity for model the data of an item annotated
    // with the [`Reflexion`] derive macro
    let fields_struct = fields.iter()
        .map( |(_vis, ident, typ, _attrs)| 
            {
                let i = ident.to_string();
                let t = get_field_type_as_string(typ);
                quote! { hm.insert(#i, #t); }
            }
    );
    
    let quote = quote! {
        impl arcane::reflexion::StructReflexion for #ty {
            
            /// Returns the identifier of a struct as a string slice
            fn get_struct_name<'a>(&'a self) -> &'a str {
                #ty_str
            }

            /// Returns a collection of Key Value pairs with the identifier of the
            /// struct's fields and the type of every field.
            fn get_stuct_fields<'a>(&'a self) -> std::collections::HashMap<&'a str, &'a str> {
                let mut hm = std::collections::HashMap::new();
                #(#hm_f)*;
                hm
            }

            ///
            fn get_info<'a>(&'a self) -> StructInfo<'a> {
                // for field in 
                arcane::reflexion::StructInfo {
                    name: #ty_str,
                    fields: vec![
                        arcane::reflexion::Field {
                            visibility: "", 
                            name: "", 
                            typ: "", 
                            attrs: &[arcane::reflexion::Attr]
                        }
                    ]
                }
            }
        }
    };

    quote.into()
}


/// TODO Code the get_enum_variants (filter_variants or whatever)
/// 
/// TODO Refactor them into a real helper struct
/// 
/// Helper for generate the fields data for the Custom Derives Macros
fn filter_fields(fields: &Fields) -> Vec<(Visibility, Ident, Type, Vec<Attribute>)> {
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
fn get_field_type_as_string(typ: &Type) -> String {
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