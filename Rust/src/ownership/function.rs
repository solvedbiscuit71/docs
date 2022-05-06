fn take_ownership(string: String) {
  println!("String: {}", string);
}
/* Since, the ownership is moved the memory is freed here.  */

fn make_copy(x: i32) {
  println!("Number: {}", x);
}

fn main() {
  let string = String::from("Rust"); // value comes into scope.

  take_ownership(string); 
  /*
    Here, string's value is moved to `take_ownership`
    it's not longer valid in this scope.
  */

  let x = 5;

  make_copy(x);
  /*
    Here, number's value is copied (instead of move) to `make_copy`
    so it's okay to used within this scope.
  */
}
/*
  !IMPORTANT:
  1. passing a parameter can move ownership
  2. return value can move ownership.
*/