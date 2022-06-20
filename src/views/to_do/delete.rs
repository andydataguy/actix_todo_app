use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use super::utils::return_state;

use crate::to_do::to_do_factory;
use crate::state::read_file;
use crate::json_serialization::item_schema::ItemSchema;
use crate::processes::process_input;

/// This function deletes an item by taking a to do item struct wrapped in jSON format and returns an HTTP response.
///
/// # Arguments
/// * to_do_item (web::Json<ToDoItem>): An individual `ItemSchema` type wrapped into a JSON struct.
///
/// # Returns
/// (HttpResponse): response body to be passed to the viewer.

pub async fn delete(to_do_item: web::Json<ItemSchema>) -> HttpResponse {
  let state: Map<String, Value> = read_file(String::from("./state.json"));

  // Extracts a reference from our ItemSchema struct to the title and status for use in the to_do_factory function
  let title: String = to_do_item.title.clone();
  let status: String = to_do_item.status.clone();

  match to_do_factory(&status, title) {
    Err(_item) => return HttpResponse::BadRequest().json(
                                                format!("{} not accepted", status)),
    Ok(item) => process_input(item, String::from("delete"), &state)
  }
  return HttpResponse::Ok().json(return_state())
}