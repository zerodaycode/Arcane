//! Library for the implementation of the macro-driven `Arcane` features

extern crate proc_macro;

use std::collections::HashMap;

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

    // let field_idents = fields.iter()
                //     .map( |(_vis, ident)|
                //         {
                //             let i = ident.to_string();
                //             quote! {
                //                 #i => Some(self.#ident.to_string())
                //             }
                //         }
                // );
    let f = fields.iter()
        .map( |(_vis, ident)| 
            {
                let i = ident.to_string();
                quote! {
                    hm.push(#i, #i);
                }
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
        impl arcane::reflexion::StructReflexion for #ty {
            
            /// Returns the identifier of a struct as a string slice
            fn get_struct_name<'a>(&'a self) -> &'a str {
                #ty_str
            }

            /// Returns a collection of Key Value pairs with the identifier of the
            /// struct's fields and the type of every field.
            fn get_stuct_fields<'a>(&'a self) -> HashMap<String, String> {
                let hm = HashMap::new();
                #(#f),*;
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
fn filter_fields(fields: &Fields) -> Vec<(Visibility, Ident)> {
    fields
        .iter()
        .map(|field| 
            (field.vis.clone(), field.ident.as_ref().unwrap().clone()) 
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