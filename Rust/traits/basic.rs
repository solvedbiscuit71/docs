use std::fmt;

struct Person {
  name: String,
  age: u8,
}

/*
  A trait tells the Rust compiler
  about functionality a particular type has and can share with other types.

  Here, std::fmt::Display is trait for formatting in println!()
  we will implement the Display trait in Person.
*/

impl fmt::Display for Person {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{{ name: {}, age: {} }}", self.name, self.age)
  }
}

fn main() {
  let user = Person {
    name: String::from("Praveen"),
    age: 18,
  };

  /*
    Here, we can use our Person because,
    We have implemented the Display Triat in struct.
  */
  println!("user: {}", user);
}
