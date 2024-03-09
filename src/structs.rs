use crate::traits::Test;

pub struct UserTest<'a> {
    pub name: &'a str,
    pub age: i32,
}

impl<'a> UserTest<'a> {
    pub fn new(name: &'a str, age: i32) -> Self {
        Self { name, age }
    }
}

impl<'a> Test for UserTest<'a> {
    fn print_test(&self) -> &str {
        println!("Name: {}", self.name);
        self.name
    }
}
