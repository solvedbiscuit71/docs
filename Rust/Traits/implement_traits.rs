use std::cmp::PartialEq;

struct Point<T> {
  x: T,
  y: T
}

impl<T> Point<T> {
  fn new(x: T, y: T) -> Point<T> {
    Point { x, y }
  }
}

impl<T: PartialEq> PartialEq for Point<T> {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

fn main() {
  let p1 = Point::new(2, 5);
  let p2 = Point::new(2, 3);

  // NOTE: p1 & p2 should be of same type i.e Point<i32>
  if p1 == p2 {
    println!("They are same");
  } else {
    println!("They are not same");
  }
}
