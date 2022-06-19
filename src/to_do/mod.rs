pub mod structs;
pub use structs::done::Done;
pub use structs::pending::Pending;

/// To do item variations based on its status.
/// 
/// ### enum ItemTypes
/// * Pending(structs::pending::Pending): To-do item with 'Pending' status.
/// * Done(structs::done::Done): To-do item with 'Done' status.

pub enum ItemTypes {
  Pending(Pending),
  Done(Done)
}

/// Function that handles the business logic for creating new to do items and
///  labeling items by status. Returns an error if the given status is not found
/// 
/// ### Arguments
/// * item_type (&String): The type of to-do item to create based on desired status.
/// * item_title (String): A descriptive title of the to-do item.
/// 
/// ### Returns
/// (Result<ItemTypes, &'static str>): Returns the created to-do item with proper matching


pub fn to_do_factory(item_type: &String, item_title: String) 
  -> Result<ItemTypes, &'static str> {

    if item_type == "pending" {
      let pending_item = Pending::new(item_title);
      Ok(ItemTypes::Pending(pending_item))
    }
    else if item_type == "done" {
      let done_item = Done::new(item_title);
      Ok(ItemTypes::Done(done_item))
    }
    else {
      Err("The item type is not accepted")
    }
  }