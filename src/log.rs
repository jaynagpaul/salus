use crate::Salus;

impl Salus {
    /// Debugging log function
    pub fn debug(&self, msg: &str) {
        println!("{}", msg);
    }

    /// Warning log function
    pub fn warn(&self, msg: &str) {
        println!("{}", msg);
    }
}
