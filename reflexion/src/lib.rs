
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
    fn get_stuct_fields<'a>(&'a self) -> HashMap<String, String>;
}