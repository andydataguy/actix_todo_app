/// This base struct is meant to provide the key attributes for any 
/// other todo items that we create. We separate the base struct from the
/// other status todo items for easier maintenance and greater flexibility.
/// 
/// # Attributes
/// * title (string): Define the todo item with a descriptive title
/// * status (string): Define the status of the todo item

pub struct Base {
  pub title: String,
  pub status: String,
}

impl Base {

/// This impl block takes two arguments and returns a newly 
/// initialized Base struct (to-do item). 
/// 
/// # Arguments
/// * input_title (&str): Input the desired name for the todo item
/// * input_status (&str): Input the current status for the todo item
/// 
/// # Returns 
/// (Base): A new Base struct with the desired title and status


  pub fn new(input_title: &str, input_status: &str) -> Base {
    return Base {title: input_title.to_string(), 
                 status: input_status.to_string()}
  }
}