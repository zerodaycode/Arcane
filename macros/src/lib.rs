//! Library for the implementation of the macro-driven `Arcane` features

extern crate proc_macro;

use proc_macro::TokenStream as CompilerTokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    DeriveInput, Fields, Visibility
};



#[proc_macro_derive(Reflexion)]
pub fn reflexion_struct_details(input: CompilerTokenStream) -> CompilerTokenStream {
    // Getting data from the AST
    let ast: DeriveInput = syn::parse(input).unwrap();
    let ty = ast.ident;
    let ty_str = ty.to_string();

    // Recovers the identifiers of the struct's members, and checks that the derive
    // macro it's only applied to structs
    let fields = filter_fields(
        match ast.data {
            syn::Data::Struct(ref s) => &s.fields,
            _ => return syn::Error::new(
                ty.span(), 
                "Reflection only works with structs"
            )
            .to_compile_error()
            .into()
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
    
    let quote = quote! {
        /// Returns the identifier of a struct as a string slice
        impl arcane::reflexion::StructReflexion for #ty {
            fn get_struct_name<'a>(&'a self) -> &'a str {
                #ty_str
            }

            
        }
    };

    quote.into()
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