use serde_json::Map;
use serde_json::value::Value;

use crate::state::write_to_file;


/// Trait for deleting a todo item 
pub trait Delete {

/// Function to delete a new todo item. 
/// 
/// ### Arguments
/// * title(&String): Provide a descriptive title for the todo item
/// * state(&mut serde::value::Value): The loaded value from the state file
/// 
/// ### Returns
/// None: Currently prints confirmation of `delete` function completion

  fn delete(&self, title: &String, state: &mut Map<String, Value>) {

    state.remove(title);
    write_to_file(String::from("./state.json"), state);
    println!("\n\nThe to do item {} has been deleted \n\n", title);

  }
}