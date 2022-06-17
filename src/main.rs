mod state;
mod to_do;
mod processes;

use std::env;
use state::read_file;
use serde_json::value::Value;
use serde_json::Map;
use to_do::to_do_factory;
use processes::process_input;

fn main() {
  // get the arguments from the environment and put them into a vector of strings
  let args: Vec<String> = env::args().collect();

  // defines command and title from the user inputs
  let command: &String = &args[1];
  let title: &String = &args[2];

  // read data from the JSON file to see the current saved to do items
  let state: Map<String, Value> = read_file("./state.json");

  // we define status outside the match statement to avoid error due to status falling out of scope
  let status: String;

  // check to see if the title is already there, sets status to pending if not
  match &state.get(*&title) {
    Some(result) => {
      status = result.to_string().replace('\"', ""); 
    }
    None => {
      status = "pending".to_string();
    }
  }

  let item = to_do_factory(&status, title).expect(&status);
  process_input(item, command.to_string(), &state);
}




// fn main() {

//   // collects the environment arguments passed in by the user and puts into a vector of strings
//   let args: Vec<String> = env::args().collect(); 

//   // defines the commands from the environment
//   let status: &String = &args[1];
//   let title: &String = &args[2];

//   // read the json file and print it to command line
//   let mut state: Map<String, Value> = read_file("./state.json");
//   println!("State: {:?}", state);

//   // insert the title and status into the map
//   state.insert(title.to_string(), json!(status));

//   // write the map to the JSON file
//   write_to_file("./state.json", &mut state);

//   // creates a pending item called wash the laundry
//   let to_do_item: Result<ItemTypes, &'static str> 
//     = to_do_factory("pending", "wash the laundry");

//   match to_do_item.unwrap() {
//     ItemTypes::Pending(item) => item.create(&item.super_struct.title),
//     ItemTypes::Done(item) => println!("it's a done item with the title: {}", item.super_struct.title)
//   }
// }