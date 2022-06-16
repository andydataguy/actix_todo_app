mod to_do;

use to_do::ItemTypes;
use to_do::to_do_factory;
use to_do::structs::traits::create::Create;

fn main() {
  let to_do_item: Result<ItemTypes, &'static str> =
    to_do_factory("pending", "wash the dishes");

// Update this match statement with any new status's from Factory
  match to_do_item.unwrap() {
    ItemTypes::Pending(item) 
      => item.create(&item.super_struct.title),
    ItemTypes::Done(item) 
      => println!("The {} item is completed!", item.super_struct.title),
  }
}

