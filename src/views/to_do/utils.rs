use serde_json::value::Value;
use serde_json::Map;
use crate::state::read_file;
use crate::to_do::{ItemTypes, to_do_factory};
use crate::json_serialization::to_do_items::ToDoItems;


/// Gets all the to do items from the state JSON file and processes them to be serialized into JSON.
/// Returns a `ToDoItems` struct which contains a count and list of each item separated by status. 
///
/// ### Arguments
/// None
///
/// ### Returns
/// * (ToDoItems): Contains all to do items from the `state.json` file
/// separated by status. It also includes a count of how many items of each status is contained. 

pub fn return_state() -> ToDoItems {
  let state: Map<String, Value> = read_file(String::from("./state.json"));

  let mut array_buffer = Vec::new();

  for (key, value) in state {
    let item_type: String = String::from(value.as_str().unwrap());
    let item: ItemTypes = to_do_factory(&item_type, key).unwrap();
    array_buffer.push(item);
  }
  return ToDoItems::new(array_buffer);
}