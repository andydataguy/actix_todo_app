use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

use crate::state::write_to_file;

/// Trait for creating a new todo item (currently with no status) 
pub trait Create {

/// Function to create a new todo item. 
/// Add status update functionality later. 
/// 
/// # Arguments
/// * title(&String): Provide a descriptive title for the todo item
/// * status(&String): Provide a status for the todo item
/// * state(&mut serde::value::Value): The loaded value from the state file
/// 
/// # Returns
/// None: Currently prints confirmation of `create` function completion

  fn create(&self, title: &String, status: &String, state: &mut Map<String, Value>) {

    // inserts the title and status from the map to be delivered as JSON to the state file
    state.insert(title.to_string(), json!(status));

    write_to_file(String::from("./state.json"), state);
    println!("\n\nThe to do item {} has been created \n\n", title);

  }
}