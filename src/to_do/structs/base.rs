use serde::Serialize;

/// This base struct is meant to provide the key attributes for any 
/// other todo items that we create. We separate the base struct from the
/// other status todo items for easier maintenance and greater flexibility.
/// 
/// ### Attributes
/// * title (string): Define the todo item with a descriptive title
/// * status (string): Define the status of the todo item

#[derive(Serialize)]
pub struct Base {
  pub title: String,
  pub status: String,
}

impl Base {

/// Function takes two arguments (title/status) and returns a newly 
/// initialized Base to do item struct. 
/// 
/// ### Arguments
/// * input_title (String): Input the desired name for the todo item
/// * input_status (String): Input the current status for the todo item
/// 
/// ### Returns 
/// (Base): A new Base struct with the desired title and status


  pub fn new(input_title: String, input_status: String) -> Base {
    return Base {title: input_title.to_string(), 
                 status: input_status.to_string()}
  }
}
