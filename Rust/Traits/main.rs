/*
  A Trait tells the Rust compiler,
  about functionality a particular type has and can share with other types.

  Example, Display functionality is trait for formatting in println!()
  which we implemented in ../Generics/generics_in_struct.rs
*/

// This is a custom trait which we intend to use in struct for summary the entire struct.
trait Summary {
  fn get_summary(&self) -> String;
}

struct Person {
  name: String,
  age: u32,
}

impl Person {
  fn new(name: &str, age: u32) -> Person {
    Person { name: String::from(name), age }
  }
}

impl Summary for Person {
  fn get_summary(&self) -> String {
    String::from(format!(
      "My name is {} and I am {} years old.",
      self.name,
      self.age
    ))
  }
}

fn about<T: Summary>(per: &T) {
  println!("About:\n{}", per.get_summary());
}

fn main() {
  let per = Person::new("Praveen",18);

  // about() funtion will accept 'per' because Person is implemented.
  about(&per);
}
