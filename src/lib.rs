/*
VALKYRIE by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module for
/// handling errors.
pub use modules::err::*;

/// Re-exporting the module
/// for lexing code.
pub use modules::lexer::*;

/// Re-exporting the module
/// for parsing project manifests.
pub use modules::project::*;
