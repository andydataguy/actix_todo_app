use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

/// This function reads a JSON file from storage.
/// It takes the file path as a string and creates a Map that can be easily serialized into JSON
///
/// ### Arguments
/// * file_name (String): the path to the file being read
///
/// ### Returns
/// (Map<String, Value>): returns the contents of the file as a Map
pub fn read_file(file_name: String) -> Map<String, Value> {

  let mut file = File::open(file_name).unwrap(); 

  // Currently using the unwrap function for simplicity. 
  // If there is an error here. There's no point in continuing the program. 

  // buffer to hold the file contents
  let mut buffer = String::new(); 

  // reads the file to our buffer (type String)
  file.read_to_string(&mut buffer).unwrap(); 

  // converts string from buffer into a JSON value 
  let json: Value = serde_json::from_str(&buffer).unwrap(); 

  // JSON value must be an object in order to be returned within a Map
  // Value is then cloned because if not we'd be stuck with a shared reference.
  let state: Map<String, Value> = json.as_object().unwrap().clone(); 
  return state;
}

/// This function writes a JSON map to file on disk.
///
/// ### Arguments
/// * file_name (String): the path to the file being read
/// * state (&mut Map<String, Value>): the data being written to disk
///
/// ### Returns
/// None
pub fn write_to_file(file_name: String, state: &mut Map<String, Value>) {

  // Macro converts Map to JSON value 
  let new_buffer = json!(state); 

  // turns the JSON value into a string and writes it to the file
  fs::write(file_name, new_buffer.to_string()).expect("Unable to write file"); 
}