mod state;
mod to_do;

use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};

use to_do::ItemTypes;
use to_do::to_do_factory;
use to_do::structs::traits::create::Create;

fn main() {

  // collects the environment arguments passed in by the user and collects to a vector of strings
  let args: Vec<String> = env::args().collect(); 

  // defines the commands from the environment
  let status: &String = &args[1];
  let title: &String = &args[2];

  // read the json file and print it to command line
  let mut state: Map<String, Value> = read_file("./state.json");
  println!("State: {:?}", state);

  // insert the title and status into the map
  state.insert(title.to_string(), json!(status));

  // write the map to the JSON file
  write_to_file("./state.json", &mut state);

  // creates a pending item called wash the laundry
  let to_do_item: Result<ItemTypes, &'static str> 
    = to_do_factory("pending", "wash the laundry");

  match to_do_item.unwrap() {
    ItemTypes::Pending(item) => item.create(&item.super_struct.title),
    ItemTypes::Done(item) => println!("it's a done item with the title: {}", item.super_struct.title)
  }
}




// mod to_do;

// use to_do::ItemTypes;
// use to_do::to_do_factory;
// use to_do::structs::traits::create::Create;

// fn main() {
//   let to_do_item: Result<ItemTypes, &'static str> =
//     to_do_factory("pending", "wash the dishes");

// // Update this match statement with any new status's from Factory
//   match to_do_item.unwrap() {
//     ItemTypes::Pending(item) 
//       => item.create(&item.super_struct.title),
//     ItemTypes::Done(item) 
//       => println!("The {} item is completed!", item.super_struct.title),
//   }
// }

