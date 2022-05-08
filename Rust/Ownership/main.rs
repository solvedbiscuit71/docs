/*
  Rust doesn't have a garbage collector instead have the concept of
  ownership & borrowing.

  - Each and Every value has a only owner at time.
  - When owner goes out-of-scope (end of block) value is delete (drop) automatically.

  ```
  let name = String::from("Praveen");
  ```
  
  Here, the owner of "Praveen" is name.
  But,

  ```
  let new_name = name;
  ```

  Now, the value (ownership) is more towards new_name.
*/

// Here, let name = new_name; So ownership is passed & value will be delete after fn ends.
fn greet(name: String) {
  println!("Hola, {}", name);
}

// Here, let name = &another_name; So the value is borrowed instead of move.
fn greet_borrow(name: &String) {
  println!("Hello, {}", name);
}

fn main() {
  let name = String::from("Praveen");
  let new_name = name;
  // below, "name" is not valid anymore

  greet(new_name);
  // below, "new_name" is not valid

  let another_name = String::from("sovledbiscuit71");
  greet_borrow(&another_name);

  println!("I can use another_name here, '{}' ", another_name);
}