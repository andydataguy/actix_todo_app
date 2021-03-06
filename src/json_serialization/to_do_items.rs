use std::vec::Vec;

use actix_web::{Responder, Error, HttpResponse, HttpRequest};
use serde::Serialize;
use futures::future::{ready, Ready};

use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;

/// This struct takes a vector of `Base` structs and serializes them into JSON.
/// It expects a mapping of each item title/status and returns a list of each item
/// as well as a count of how many items are included for each status.
/// Uses the `Serialize` macro to enable the Struct attributes to be converted into JSON for easy use.
///
/// ### Parameters
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

/// This function performs the logic needed to build a `ToDoItems` struct from the 
/// list of all the `Base` structs in storage to be serialized into JSON.
/// It utilizes a match statement to separate the retrieved items by status, 
/// and then uses a `len()` method to count the total number of items for each status.
///
/// ### Arguments
/// * input_items (Vec<ItemTypes>): the saved to do items super structs to be packaged
///
/// ### Returns
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

impl Responder for ToDoItems {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>; 

/// This function gets fired when the struct is being returned in an actix view.
/// Includes a type alias which denotes that the future is immediately ready with a value.
/// Inside the Future type is a `Result` which can be either an HttpResponse or an Error.
///
/// ### Arguments
/// * _req (&HttpRequest): the request belonging to the view
///
/// ### Returns
/// * (Self::Future): an OK HTTP response with the serialized struct in the body

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();
    ready(Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(body)))
  }
}