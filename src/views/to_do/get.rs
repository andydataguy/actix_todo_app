use actix_web::Responder;
use super::utils::return_state;

/// This view gets all of the saved to do items that are stored in the state.json file 
/// and returns them to the client as JSON
///
/// ### Arguments
/// None
///
/// ### Returns
/// * (web::Json): all of the stored to do items from the state.json file

pub async fn get() -> impl Responder {
  return return_state()
}