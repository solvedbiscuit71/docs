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

impl Point {
  fn get_x(self: &Self) -> i32 {
    self.x
  }

  fn get_y(&self) -> i32 {
    self.y
  }

  /* 
   * This is a assoicate function (Note: no &self here)
   * This is used as an constructor.
   */
  fn new(x: i32, y: i32) -> Point {
    let quarant = if x > 0 && y > 0 {
      Quarant::First
    } else if x < 0 && y > 0 {
      Quarant::Second
    } else if x < 0 && y < 0 {
      Quarant::Third
    } else {
      Quarant::Fourth
    };
    Point { x, y, quarant }
  }
}

fn main() {
  let p1 = Point::new(-15, 30);

  println!("The point p1 is in {:?} quarant", p1.quarant);
}