use super::base::Base;

/// The `Pending` struct defines a to-do item which is meant to be considered "pending"
/// 
/// # Attributes
/// * super_struct (Base): Inherited struct from the Base struct to house key attributes

pub struct Pending {
  pub super_struct: Base
}

impl Pending {

/// The `Pending` impl block initializes a new to-do Struct with a pre-configured status. 
/// It automatically locks down the status to "pending." 
/// This avoids errors from inconsistent naming convention.
/// 
/// # Arguments
/// * input_title (&str): Define the to-do item with a descriptive title
/// 
/// # Returns
/// (Pending): A new `Pending` struct to represent a to-do item with the status of "pending"

  pub fn new(input_title: &str) -> Pending {
    let base: Base = Base::new(input_title, "pending");
    return Pending {super_struct: base};
  }
}