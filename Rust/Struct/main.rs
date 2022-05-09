/*
  We use enum for defining types with different variants.
  But, what if we want to store different types into one (grouping).
*/

// Here, derive(Debug) is used to format the enum in {:?}
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

fn main() {
  let p1 = Point {
    x: 32,
    y: -15,
    quarant: Quarant::Fourth
  };

  println!("P1 ({}, {}) which is in {:?}", p1.x, p1.y, p1.quarant);
}