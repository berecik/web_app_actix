pub struct Base {
    pub title : String,
    pub status : String,
}

impl Base {
    pub fn new(title : &str, status : &str) -> Base {
        Base {
            title: title.to_string(),
            status: status.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{} ({})", self.title, self.status);
    }
}