//! Library for the implementation of the macro-driven `Arcane` reflective features

extern crate proc_macro;
extern crate arcane_ops;


// use arcane_reflexion::StructInfo;
use arcane_ops::macros::StructParser;

use proc_macro::TokenStream as CompilerTokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{
    Fields, Type, Visibility, Attribute
};


#[proc_macro_derive(StructInfo)]
pub fn reflexion_struct_details(input: CompilerTokenStream) -> CompilerTokenStream {

    // Check that the derive macro it's properly applicated
    let ast: syn::DeriveInput = syn::parse(input.clone()).unwrap();
    match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => return syn::Error::new(
            ast.ident.span(), 
            "StructInfo only works with structs"
        )
        .to_compile_error()
        .into()
    };

    let truct = syn::parse::<StructParser>(input);

    if truct.is_err() {
        return truct.err().unwrap().into_compile_error().into()
    }

    // No errors detected on the parsing, so we can safely unwrap the parsed result
    let _struct = truct.unwrap();
    let ty = &_struct.ident;

    let ty_str = &ty.to_string();

    // Parsing the data of the fields of the struct
    let fields = filter_fields(&_struct.fields);

    // Generates the tokens for create the relationship between the fields and it's
    // declared types
    let hm_f = fields
        .iter()
        .map( |(_vis, ident, typ, _attrs)| 
            {
                let i = ident.to_string();
                let t = get_field_type_as_string(typ);
                quote! { hm.insert(#i, #t); }
            }
        );

    // Getting the attrs attached to the struct
    let st_attrs = _struct
        .clone()
        .attributes
        .into_iter()
        .map( |attr|
            {
                let att = attr.to_token_stream().to_string();
                let path = attr.path.to_token_stream().to_string();
                let tokens = attr.tokens.to_string();
                
                quote! {
                    arcane_reflexion::Attribute {
                        attr: #att,
                        path: #path,
                        tokens: #tokens
                    }
                }
            }
        );


    // Generates the [`StructInfo`] entity for model the data of an item annotated
    // with the [`Reflexion`] derive macro
    let struct_info_fields = fields
        .iter()
        .map( |(_vis, _ident, _typ, _attrs)| 
            {
                let vis = _vis.to_token_stream().to_string();
                let name = _ident.to_string();
                let typ = get_field_type_as_string(_typ);
                let attrs = _attrs.iter()
                    .map( |attr|
                        {
                            let path = attr.path.to_token_stream().to_string();
                            quote! {
                                arcane_reflexion::Attribute {
                                    path: #path
                                }
                            }
                        }
                    );

                quote! {
                    arcane_reflexion::Field::new( 
                        #vis,
                        #name,
                        #typ,
                        vec![
                            #(#attrs),*
                        ]
                    )
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
            fn get_stuct_fields<'a>(&'a self) -> std::collections::HashMap<&'a str, &'a str> {
                let mut hm = std::collections::HashMap::new();
                #(#hm_f)*;
                hm
            }

            /// Returns an [`arcane::reflexion::StructInfo`] entity that contains
            /// runtime reflexive info about `Self`.
            fn get_info<'a>(&'a self) -> arcane::reflexion::StructInfo {
                arcane::reflexion::StructInfo {
                    name: #ty_str,
                    fields: vec![
                        #(#struct_info_fields),*
                    ],
                    attrs: vec![
                        #(#st_attrs),*
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
/// Helper for destructure de [`syn::Fields`] into a [`Vec`] of tuples
/// that holds the attributes of every field.
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