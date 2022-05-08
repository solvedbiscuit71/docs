/*
  an string slice (&str) is a string literal.

  ```
  let name: &str = "praveen";
  ```
  Which is an reference slice of a String.
*/

fn take_ownership(s: String) {}

fn main() {
  let name = String::from("John Carter");
  let first_name = &name[0..4];
  /*
    Here, first_name is valid as long as name is valid.

    ```
    take_ownership(name);
    ```

    Will cause error.
  */

  // take_ownership(name);
  println!("First name is {} (name)", first_name);
}