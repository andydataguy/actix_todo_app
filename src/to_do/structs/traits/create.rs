// Create scaffolding for CRUD trait implementations. 
// Test that files all connect properly before implementing business logic

/// Trait for creating a new todo item (currently with no status) 
pub trait Create {

/// Function to create a new todo item. 
/// Add status update functionality later. 
/// 
/// # Arguments
/// * title(&str): Provide a descriptive title for the todo item
/// 
/// # Returns
/// None: Currently prints confirmation of `create` function completion

  fn create(&self, title: &str) {
    println!("Todo Item {} is being created!", title);
  }
}