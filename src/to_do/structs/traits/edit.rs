// Create scaffolding for CRUD trait implementations. 
// Test that files all connect properly before implementing business logic

// Start with making functions that implements changes for each status (done and pending)

/// Trait for editing the status of todo items 
pub trait Edit {

/// Function to change the status of a todo item to "done" 
/// 
/// # Arguments
/// * title(&str): Provide title of the todo item to be changed to "done"
/// 
/// # Returns
/// None: Currently prints confirmation of `set_to_done` function completion

  fn set_to_done(&self, title: &str) {
    println!("Todo Item {} is being marked as done!", title);
  }

/// Function to change the status of a todo item to "pending" 
/// 
/// # Arguments
/// * title(&str): Provide title of the todo item to be changed to "pending"
/// 
/// # Returns
/// None: Currently prints confirmation of `set_to_pending` function completion

  fn set_to_pending(&self, title: &str) {
    println!("Todo Item {} is being marked as pending!", title);
  }
}