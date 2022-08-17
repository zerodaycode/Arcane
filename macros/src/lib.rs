//! Library for the implementation of the macro-driven `Arcane` features

extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream as CompilerTokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{
    DeriveInput, Fields, Type, Visibility
};



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

    let fields_name_type = fields_name_type(ast_data);

    let hm_f = fields.iter()
        .map( |(_vis, ident, typ)| 
            {
                let i = ident.to_string();
                let t = get_field_type_as_string(typ);
                quote! {
                    hm.push(#i, #i);
                }
            }
    );

    let _field_idents = fields.iter()
        .map( |(_vis, ident, _typ)|
            {
                let i = ident.to_string();
                quote! {
                    #i => Some(self.#ident.to_string())
                }
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
            fn get_stuct_fields<'a>(&'a self) -> HashMap<String, String> {
                let hm = HashMap::new();
                #(#hm_f),*;
                hm
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
fn filter_fields(fields: &Fields) -> Vec<(Visibility, Ident, Type)> {
    fields
        .iter()
        .map( |field| 
            (
                field.vis.clone(), 
                field.ident.as_ref().unwrap().clone(),
                field.ty.clone()
            ) 
        )
        .collect::<Vec<_>>()
}

fn fields_name_type(fields: &Fields) -> HashMap<String, String> {
    fields
        .iter()
        .map( |field| {
            return (
                field.ident.as_ref().unwrap().clone().to_string(), 
                "".to_string()
            )
        }).collect::<HashMap<String, String> >()
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