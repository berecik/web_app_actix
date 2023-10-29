use super::base::Base;

trait State {
        fn print(&self, super_struct : &Base);
        fn to_string(&self, super_struct : &Base) -> String;
}

impl State for Base {
        fn print(&self, super_struct : &Base) {
                println!("{}", self.to_string(super_struct));
        }
        fn to_string(&self, super_struct : &Base) -> String {
                return format!("{} ({})", super_struct.title, super_struct.status);
        }
}
        }
        fn to_string(&self, super_struct : &Base) -> String {
                return format!("{} ({})", super_struct.title, super_struct.status);
        }
}

