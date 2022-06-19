use serde::Deserialize;

/// This struct contains two parameters including the title and status of a to do item. 
/// Makes it easier to extract data from the serialized `ToDoItems` struct.
///
/// ### Parameters
/// * title(String): The title of a to do item
/// * status(String): The status of a to do item

#[derive(Deserialize)]
pub struct ItemSchema {
  pub title: String,
  pub status: String,
}