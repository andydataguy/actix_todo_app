use actix_web::{web, HttpResponse};
use serde_json::value::Value;
use serde_json::Map;

use super::utils::return_state;

use crate::to_do::to_do_factory;
use crate::state::read_file;
use crate::json_serialization::item_schema::ItemSchema;
use crate::processes::process_input;

/// This function takes a to do item struct wrapped in JSON format to be edited and returned as an HTTP response.
///
/// ### Arguments
/// * to_do_item(web::Json<ToDoItem>): An individual `ItemSchema` type wrapped into a JSON struct. 
///
/// ### Returns
/// (HttpResponse): an HTTP response containing either the edited item or an error message if status is not found.

pub async fn edit(to_do_item: web::Json<ItemSchema>) -> HttpResponse {
  let state: Map<String, Value> = read_file(String::from("./state.json"));

  let title_reference: &String = &to_do_item.title.clone(); 

  // Extracts a reference to the title and clones it for use in the to_do_factory function
  let title: String = to_do_item.title.clone();

  let status: String;

  // Checks our state.json file to see if the to do item actually exists. It returns a 404 error message if not.
  // Match reference to state by calling get a reference to the title to access the status
  match &state.get(title_reference) {
    Some(result) => {
      
      // Takes the extracted item status, converts it to a string, and removes the `\"` characters
      status = result.to_string().replace('\"', "");
    }
    None => {
      return HttpResponse::NotFound().json(
                                     format!("{} not found in state", title_reference))
    }
  }

  // Checks if the returned status is the same as the status originally passed into the view
  // If the status is the same. Then it simply returns the current status as there is no change to be made.
  if &status == &to_do_item.status {
    return HttpResponse::Ok().json(return_state())
  }

  match to_do_factory(&status, title) {

    // returns a BadRequest message if the given status type is not found in the `to_do_factory()` function
    Err(_item) => return HttpResponse::BadRequest().json(
                                                  format!("{} not accepted", status)),
    Ok(item) => process_input(item, String::from("edit"), &state)
  }

  // If the request is successful then returns a 200 status with the state in the body
  return HttpResponse::Ok().json(return_state())
}