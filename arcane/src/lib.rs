//! The 'Arcane' project
//! 
//! Arcane project was born to provide the developer some facilities
//! about obtain runtime info on certain pices of code, known in other languages
//! as the `reflexion` concept, which is the ability of the code to introspect
//! itself to get details about itself.
//! 
//! Later we added another goal, generate bits of code that 
//! most of the time are boilerplate code, related with that info, like for example,
//! generate getters and setters when you don't want to publicly expose the
//! internal details about a type. 
//! 
//! Then we desired to refactor some code that we written since a time
//! ago into a collection of utils when work with macros. Parsing data, 
//! retrieving fields, types, idents...
//! 


/// Project mods
pub extern crate arcane_macros;


// The game of reexports
mod types;
// pub use arcane_macros;

/// Provides access to the public API of reflexion
pub mod reflexion {
    pub use arcane_macros::*;
    pub use crate::types::*;
}