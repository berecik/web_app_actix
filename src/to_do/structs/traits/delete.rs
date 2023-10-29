pub trait Get {
    fn create(&self, title : &str) {
        println!("{} is being deleted", title);
    }
}