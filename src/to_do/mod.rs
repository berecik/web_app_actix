pub mod structs;

use structs::done::Done;
use structs::pending::Pending;

pub enum ItemTypes {
    Done(Done),
    Pending(Pending),
}

pub fn to_do_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    match item_type {
        "done" => {
            let done = Done::new(item_title);
            Ok(ItemTypes::Done(done))
        },
        "pending" => {
            let pending = Pending::new(item_title);
            Ok(ItemTypes::Pending(pending))
        },
        _ => Err("Invalid item type")
    }
}