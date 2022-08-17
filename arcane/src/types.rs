//! Here resides the core types and traits of `Arcane`



/// Defines the base reflection actions
pub trait Reflexion {}


/// Gives a user defined type the ability of introspect itself
/// and provide the caller info about the type at runtime.
pub trait StructReflexion {
    fn get_struct_name<'a>(&'a self) -> &'a str;
}