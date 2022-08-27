
//! The 'Reflexion' crate of the `Arcane` project
//! 
//! `Arcane-Reflexion` project was born to provide the developer some facilities
//! about obtain runtime info on certain pices of code, known in other languages
//! as the `reflexion` concept, which is the ability of the code to introspect
//! itself to get details about itself.

use std::collections::HashMap;


/// Defines the base reflection actions
pub trait Reflexion {}


/// Gives a user defined type the ability of introspect itself
/// and provide the caller info about the type at runtime.
pub trait StructReflexion {
    /// Returns the identifier of a struct as a string slice
    fn get_struct_name<'a>(&'a self) -> &'a str;

    /// Returns a collection of Key Value pairs with the identifier of the
    /// struct's fields and the type of every field.
    fn get_stuct_fields<'a>(&'a self) -> HashMap<&'a str, &'a str>;

    /// Returns an [`arcane::reflexion::StructInfo`] entity that contains
    /// runtime reflexive info about `Self`.
    fn get_info<'a>(&'a self) -> StructInfo;
}


/// Type for wrapping all the info available about an implementor
/// of [`StructReflexion`], offering it as 
pub struct StructInfo<'a> {
    pub name: &'a str,
    pub fields: Vec<Field<'a>>,
    pub attrs: Vec<Attribute<'a>>
}

impl<'a> StructInfo<'a> {
    pub fn new(
        name: &'a str, 
        fields: Vec<Field<'a>>, 
        attrs: Vec<Attribute<'a>>
    ) -> Self {
        Self {
            name: name,
            fields: fields,
            attrs: attrs
        }
    }
}


/// The `Arcane` reflexive version of the [`syn::Fields`], providing
/// info about a field of a struct, mostly as is string repr.
#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub visibility: &'a str,
    pub name: &'a str,
    pub typ: &'a str,
    pub attrs: Vec<Attribute<'a>>
}


/// The runtime reflexive info of attribute attached to a element of code
/// 
#[derive(Debug, Clone)]
pub struct Attribute<'a> {
    pub attr: &'a str,
    pub path: &'a str,
    pub tokens: &'a str

}

/// Variants represents the visibility of a Rust source code item.
pub enum ItemVisibility {
    Public,
    Private
}