use super::base::Base;
use super::traits::create::Create;
use super::traits::get::Get;
use super::traits::edit::Edit;
use super::traits::delete::Delete;

/// The `Pending` struct defines a to-do item which is meant to be considered "pending"
/// 
/// ### Attributes
/// * super_struct (Base): Inherited struct from the Base struct to house key attributes

pub struct Pending {
  pub super_struct: Base
}

impl Pending {

/// The `Pending` impl block initializes a new to-do Struct with a pre-configured status. 
/// It automatically locks down the status to "pending." 
/// This avoids errors from inconsistent naming convention.
/// 
/// ### Arguments
/// * input_title (String): Define the to-do item with a descriptive title
/// 
/// ### Returns
/// (Pending): A new `Pending` struct to represent a to-do item with the status of "pending"

  pub fn new(input_title: String) -> Pending {
    let input_status = String::from("pending");
    let base: Base = Base::new(input_title, input_status);
    return Pending {super_struct: base};
  }
}

impl Get for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
impl Create for Pending {}