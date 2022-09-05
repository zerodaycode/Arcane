
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


/// Gives a user defined enum type the ability of introspect itself
/// and provide the caller info about the type at runtime.
pub trait EnumReflexion {
    /// Returns the identifier of the enum type as an &str
    fn get_name<'a>(&'a self) -> &'a str;
    /// Returns an [`arcane::reflexion::EnumInfo`] entity that contains
    /// runtime reflexive info about `Self`.
    fn get_info<'a>(&'a self) -> EnumInfo;
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
    pub visibility: ItemVisibility,
    pub name: &'a str,
    pub typ: &'a str,
    pub attrs: Vec<Attribute<'a>>
}

impl<'a> Field<'a> {
    pub fn new(
        vis: &'a str, 
        name: &'a str, 
        typ: &'a str, 
        attrs: Vec<Attribute<'a>>
    ) -> Self {
        Self { 
            visibility: match vis {
                "pub" => ItemVisibility::Public,
                &_ => ItemVisibility::Private
            },
            name: name, 
            typ: typ, 
            attrs: attrs 
        }
    }
}


#[derive(Debug, Clone)]
pub struct EnumInfo<'a> {
    pub name: &'a str,
    pub variants: Vec<VariantInfo<'a>>
}

/// Reflective data of a variant of some enum type
#[derive(Debug, Clone)]
pub struct VariantInfo<'a> {
    pub name: &'a str,
    pub fields: Vec<Field<'a>>,
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
#[derive(Debug, Clone, PartialEq)]
pub enum ItemVisibility {
    Public,
    Private
}

impl std::fmt::Display for ItemVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ItemVisibility::Public => write!(f, "Public"),
            ItemVisibility::Private => write!(f, "Private")
        }
    }
}
