// Create scaffolding for CRUD trait implementations. 
// Test that files all connect properly before implementing business logic

/// Trait for deleting a todo item 
pub trait Delete {

/// Function to delete a new todo item. 
/// 
/// # Arguments
/// * title(&str): Provide a descriptive title for the todo item
/// 
/// # Returns
/// None: Currently prints confirmation of `delete` function completion

  fn delete(&self, title: &str) {
    println!("Todo Item {} is being deleted!", title);
  }
}