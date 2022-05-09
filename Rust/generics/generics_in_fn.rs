use std::ops::Add;

// Here, _add() only takes i32, but what about floats??
fn _add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}

// Here, T: Add<Output = T> means anytype which support (implement) add (+) functionality.
fn add<T: Add<Output = T>>(n1: T, n2: T) -> T {
  n1 + n2
}

fn main() {
  println!("2.5 + 3.5 = {}", add(2.5, 3.5));
  println!("2 + 2 = {}", add(2, 2));

  // And, Obviously add(2.5, 2); will cause error.
}
