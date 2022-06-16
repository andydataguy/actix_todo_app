use super::base::Base;

/// The `Done` struct defines a to-do item which is meant to be considered "done" 
/// 
/// # Attributes
/// * super_struct (Base): Inherited struct from the Base struct to house key attributes

pub struct Done {
  pub super_struct: Base
}

impl Done {

/// The `Done` impl block initializes a new to-do Struct with a pre-configured status. 
/// It automatically locks down the status to "done." 
/// This avoids errors from inconsistent naming convention.
/// 
/// # Arguments
/// * input_title (&str): Define the to-do item with a descriptive title
/// 
/// # Returns
/// (Done): A new `Done` struct to represent a to-do item with the status of "done"

  pub fn new(input_title: &str) -> Done {
    let base: Base = Base::new(input_title, "done");
    return Done {super_struct: base};
  }
}

