use super::base::Base;
use super::traits::get::Get;
use super::traits::delete::Delete;
use super::traits::create::Create;
use super::traits::edit::Edit;

pub struct Done {
    pub super_struct : Base,
}

impl Done {
    pub fn new(title : &str) -> Done {
        let base = Base::new(title, "done");
        return Done {
            super_struct: base,
        }
    }

    pub fn print(&self) {
        println!("{}", self.super_struct.to_string());
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Create for Done {}
impl Edit for Done {}
