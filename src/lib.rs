//! The 'Arcane' project
//! 
//! Arcane project was born to provide the developer some facilities
//! about obtain runtime info on certain pices of code, known in other languages
//! as the `reflexion` concept, which is the ability of the code to introspect
//! itself to get details about itself, and/or generate bits of code that 
//! most of the time are boilerplate code, related with that info, like for example,
//! generate getters and setters when you don't want to publicly expose the
//! internal details about a type. 
//! 

extern crate proc_macro;

use proc_macro::TokenStream as CompilerTokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    DeriveInput, Fields, Visibility
};

/// Contains macros that provides runtime info about different
/// Rust pieces of code.

// mod reflexion {  // reexport macros in another crate

#[proc_macro_attribute]
pub fn reflexion_details(_meta: CompilerTokenStream, input: CompilerTokenStream) -> CompilerTokenStream {
    // Getting data from the AST
    let ast: DeriveInput = syn::parse(input.clone()).unwrap();
    let ty = ast.ident;

    // Recovers the identifiers of the struct's members
    let fields = filter_fields(
        match ast.data {
            syn::Data::Struct(ref s) => &s.fields,
            _ => return syn::Error::new(
                ty.span(), 
                "ForeignKeyable only works with Structs"
            ).to_compile_error().into()
        }
    );

    let field_idents = fields.iter()
        .map( |(_vis, ident)|
            {
                let i = ident.to_string();
                quote! {
                    #i => Some(self.#ident.to_string())
                }
            }
    );
    input.into()
}


/// Helper for generate the fields data for the Custom Derives Macros
fn filter_fields(fields: &Fields) -> Vec<(Visibility, Ident)> {
    fields
        .iter()
        .map(|field| 
            (field.vis.clone(), field.ident.as_ref().unwrap().clone()) 
        )
        .collect::<Vec<_>>()
}