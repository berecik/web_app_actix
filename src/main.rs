mod to_do;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;
use to_do::to_do_factory;
use to_do::ItemTypes;
use to_do::structs::traits::create::Create;

fn main() {
    let done = Done::new("shopping");
    done.print();
    let pending = Pending::new("yachting");
    pending.print();
    done.print();
    let to_do_make: Result<ItemTypes, &'static str> = to_do_factory("pending", "make");
    match to_do_make {
        Ok(item) => {
            match item {
                ItemTypes::Done(done) => {
                    done.print();
                },
                ItemTypes::Pending(pending) => {
                    pending.print();
                },
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
    let to_do_washing: Result<ItemTypes, &'static str> = to_do_factory("pending", "washing");
    match to_do_washing.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => item.print()
    }

}
