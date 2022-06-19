use super::base::Base;
use super::traits::get::Get;
use super::traits::edit::Edit;
use super::traits::delete::Delete;

/// The `Done` struct defines a to-do item which is meant to be considered "done" 
/// 
/// ### Attributes
/// * super_struct (Base): Inherited struct from the Base struct to house key attributes

pub struct Done {
  pub super_struct: Base
}

impl Done {

/// The `Done` impl block initializes a new to-do Struct with a pre-configured status. 
/// It automatically locks down the status to "done." 
/// This avoids errors from inconsistent naming convention.
/// 
/// ### Arguments
/// * input_title (String): Define the to-do item with a descriptive title
/// 
/// ### Returns
/// (Done): A new `Done` struct to represent a to-do item with the status of "done"

  pub fn new(input_title: String) -> Done {
    let input_status = String::from("done");
    let base: Base = Base::new(input_title, input_status);
    return Done {super_struct: base};
  }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}