use std::vec::Vec;

use serde::Serialize;
use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;

/// This struct takes the `Base` structs and serializes them into JSON for the client to use.
/// The `Serialize` macro enables the Struct attributes to be serialized into JSON.
///
/// # Parameters
/// * pending_items (Vec<Base>): vector containing the statuses and titles of pending items
/// * done_items (Vec<Base>): vector containing the statuses and titles of the done items
/// * pending_item_count (i8): the number of pending items stored
/// * done_item_count (i8): the number of done items stored

#[derive(Serialize)]
pub struct ToDoItems {
  pub pending_items: Vec<Base>,
  pub done_items: Vec<Base>,
  pub pending_item_count: i8,
  pub done_item_count: i8,
}

impl ToDoItems {

/// This function performs the logic to build the `ToDoItems` struct from the 
/// list of all the `Base` structs in a format ready to be serialized into JSON
///
/// # Arguments
/// * input_items (Vec<ItemTypes>): the saved to do items super structs to be packaged
///
/// # Returns
/// * (ToDoItems): Struct containing the list of items and counts organized by pending and done status

  pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
    let mut pending_array_buffer = Vec::new();
    let mut done_array_buffer = Vec::new();

    for item in input_items {
      match item {
        ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
        ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct),
      }
    }
    let done_count: i8 = done_array_buffer.len() as i8;
    let pending_count: i8 = pending_array_buffer.len() as i8;
    return ToDoItems{
      pending_items: pending_array_buffer,
      done_items: done_array_buffer,
      pending_item_count: pending_count,
      done_item_count: done_count,
    }
  }
}