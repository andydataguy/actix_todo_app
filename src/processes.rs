use serde_json::Map;
use serde_json::value::Value;

use super::to_do::ItemTypes;
use super::to_do::structs::done::Done;
use super::to_do::structs::pending::Pending;
use super::to_do::structs::traits::get::Get;
use super::to_do::structs::traits::create::Create;
use super::to_do::structs::traits::delete::Delete;
use super::to_do::structs::traits::edit::Edit;


/// Function acts as an entry point for commands sent by the server. 
/// A match statement is used to determine which command was sent and to execute the 
/// appropriate functions.
/// 
/// ### Arguments
/// * item(ItemTypes): An enum that defines the status of a to do item
/// * command(String): Desired command to perform on a particular to do item
/// * state(&serde::value::Value): The current item state pulled from storage as a `Map<String, Value>`
/// 
/// ### Returns
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
/// ### Arguments
/// * item(Pending): The pending to do item to be processed
/// * command(String): The command to be performed on the pending to do item
/// * state(&serde::value::Value): The current state of the to do item for the program
/// 
/// ### Returns
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
/// ### Arguments
/// * item(Done): The done to do item to be processed
/// * command(String): The command to be performed on the done to do item
/// * state(&serde::value::Value): The current state of the to do item for the program
/// 
/// ### Returns
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