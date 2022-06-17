use serde_json::Map;
use serde_json::value::Value;

use super::to_do::ItemTypes;
use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::structs::traits::get::Get;
use super::to_do::structs::traits::create::Create;
use super::to_do::structs::traits::delete::Delete;
use super::to_do::structs::traits::edit::Edit;


/// Input function that acts as an entry to running commands on the to do items.
/// This match statement is mapped to other match statements to provide more flexibility. 
/// When we're ready to add more status types, one can simply add a line to the match statement.
/// 
/// # Arguments
/// * item(ItemTypes): This is a to do item that will have a command completed on it
/// * command(String): Desired command to perform on a particular to do item
/// * state(&serde::value::Value): The current state of the to do item for the program
/// 
/// # Returns
/// None

pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {


  // having a match statement which matches other match statements gives us flexibility
  match item {
    ItemTypes::Pending(item) => process_pending(item, command, state),
    ItemTypes::Done(item) => process_done(item, command, state),
  }
}

/// Function that processes a command on pending to do items.
/// 
/// # Arguments
/// * item(Pending): The pending to do item to be processed
/// * command(String): The command to be performed on the pending to do item
/// * state(&serde::value::Value): The current state of the to do item for the program
/// 
/// # Returns
/// None

fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
  let mut state = state.clone();

  match command.as_str() {

    "get" => item.get(&item.super_struct.title, &state),
    "create" => item.create(&item.super_struct.title, &item.super_struct.status, &mut state),
    "delete" => item.delete(&item.super_struct.title, &mut state),
    "edit" => item.set_to_done(&item.super_struct.title, &mut state),
    _ => println!("Command: {} is not recognized", command)

  }
}

/// Function that processes a command on done to do items.
/// 
/// # Arguments
/// * item(Done): The done to do item to be processed
/// * command(String): The command to be performed on the done to do item
/// * state(&serde::value::Value): The current state of the to do item for the program
/// 
/// # Returns
/// None

fn process_done(item: Done, command: String, state: &Map<String, Value>) {
  let mut state = state.clone();

  match command.as_str() {
    "get" => item.get(&item.super_struct.title, &state),
    "delete" => item.delete(&item.super_struct.title, &mut state),
    "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
    _ => println!("Command: {} is not recognized.", command)
  }
}