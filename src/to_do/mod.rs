pub mod structs;
pub use structs::done::Done;
pub use structs::pending::Pending;

/// List of to-do item types that can be added to the to-do list. 
/// Add new types to the list of types in the ItemTypes enum. 
/// 
/// # enum ItemTypes
/// * Pending(structs::pending::Pending): To-do item with 'Pending' status.
/// * Done(structs::done::Done): To-do item with 'Done' status.

pub enum ItemTypes {
  Pending(Pending),
  Done(Done)
}

/// Factory function for creating to-do items. 
/// Add new types to the if-else statement in the factory function.
/// 
/// # Arguments
/// * item_type (&String): The type of to-do item to create based on desired status.
/// * item_title (String): A descriptive title of the to-do item.
/// 
/// # Returns
/// (Result<ItemTypes, &'templates str>): The created to-do item with proper matching


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