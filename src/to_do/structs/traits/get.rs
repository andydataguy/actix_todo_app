use std::option::Option;
use serde_json::Map;
use serde_json::value::Value;

/// Trait for retrieving todo items
pub trait Get {

/// Get function retrieves a todo item and prints information to the console
/// 
/// # Arguments
/// * title(&String): Provide title of the todo item to be retrieved
/// * status(serde::value::Value): The loaded value from the state file
/// 
/// # Returns
/// None: Currently prints confirmation of 'get` function completion or failure

  fn get(&self, title: &String, state: &Map<String, Value>) {

    let item: Option<&Value> = state.get(title);

    // manages the Option type with a match statement
    match item {
      Some(result) => {
        println!("\n\nTo do Item: {}", title);
        println!("Status: {}", result);
      },
      None => println!("To do item {} was not found", title)
    }
  }
}
