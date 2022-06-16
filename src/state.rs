use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;


// Creates a Map from files so that we can easily access the data

/// Takes filepath as a string and uses the standard library to open it.

pub fn read_file(file_name: &str) -> Map<String, Value> {

  let mut file = File::open(file_name.to_string()).unwrap(); 

  // Currently using the unwrap function for simplicity. 
  // If there is an error here. There's no point in continuing the program. 

  // buffer to hold the file contents
  let mut buffer = String::new(); 

  // reads the file to our buffer (type String)
  file.read_to_string(&mut buffer).unwrap(); 

  // converts string from buffer into a JSON value 
  let json: Value = serde_json::from_str(&buffer).unwrap; 

  // JSON value must be an object in order to be returned within a Map
  // This value is then cloned because if not we'd be stuck with a shared reference.
  let state: Map<String, Value> = json.as_object().unwrap().clone(); 
  return state;
}