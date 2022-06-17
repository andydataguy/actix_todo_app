// Create scaffolding for CRUD trait implementations. 
// Test that files all connect properly before implementing business logic

/// The structs are separated into their own file folder for better flexibility.
/// Now we can update struct functionality without dealing with changing business logic

pub mod create;
pub mod get;
pub mod edit;
pub mod delete;