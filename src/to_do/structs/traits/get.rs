// Create scaffolding for CRUD trait implementations. 
// Test that files all connect properly before implementing business logic

/// Trait for retrieving todo items
pub trait Get {

/// Get function retrieves a todo item
/// 
/// # Arguments
/// * title(&str): Provide title of the todo item to be retrieved
/// 
/// # Returns
/// None: Currently prints confirmation of 'get` function completion

  fn get(&self, title: &str) {
    println!("Todo Item {} is being fetched!", title);
  }
}