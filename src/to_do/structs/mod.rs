/// # Creating new Status options
/// There is a `base` module set to private to avoid naming conflicts.
/// Duplicate the `done` file to add additional status options.
/// Configure as desired.
/// Add newly made statuses to the `mod.rs` file in the `structs` folder.
/// Be sure to include the `pub` keyword to allow the `main.rs` file to use the new struct.

pub mod traits;
mod base; 
pub mod done;
pub mod pending;