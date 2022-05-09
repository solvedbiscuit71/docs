#[derive(Debug)]
enum Quarant {
  First,
  Second,
  Third,
  Fourth,
  Origin
}

struct Point {
  x: i32,
  y: i32,
  quarant: Quarant
}

// To implement methods to a struct we use the impl block.
impl Point {
  fn get_x(self: &Self) -> i32 {
    self.x
  }

  // Here, &self <=> self: &Self
  fn get_y(&self) -> i32 {
    self.y
  }
}

fn main() {
  let mut p1 = Point {
    x: 10,
    y: 20,
    quarant: Quarant::First
  };

  p1.x = 15;
  println!("P({}, {})",p1.get_x(), p1.get_y());
}
