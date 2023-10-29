use super::base::Base;
use super::traits::get::Get;
use super::traits::delete::Delete;
use super::traits::create::Create;
use super::traits::edit::Edit;

pub struct Pending {
    pub super_struct : Base,
}

impl Pending {
    pub fn new(title : &str) -> Pending {
        let base = Base::new(title, "pending");
        return Pending {
            super_struct: base,
        }
    }

    pub fn print(&self) {
        println!("{}", self.super_struct.to_string());
    }
}

impl Get for Pending {}
impl Delete for Pending {}
impl Create for Pending {}
impl Edit for Pending {}