use std::fmt;
use std::fmt::Display;

struct Point<X,Y> {
  x: X,
  y: Y
}

// Here, we are implementing the Display support so that we can format using "{}"
impl<X,Y> Display for Point<X,Y>
where
  X: Display,
  Y: Display
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Point ({}, {})", self.x, self.y)
  }
}

fn main() {
  let p1 = Point {
    x: 2.5,
    y: 2
  };

  println!("{}", p1);
}