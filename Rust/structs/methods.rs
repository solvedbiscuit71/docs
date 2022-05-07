struct Rect {
  width: u32,
  height: u32,
}

impl Rect {
  // NOTE: &self === self: &Self
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

fn main() {
  let mut rect = Rect {
    width: 10,
    height: 5,
  };

  println!("{}x{} is the dimension", rect.width, rect.height);

  rect.width = 15;
  println!("{}x{} is the dimension", rect.width, rect.height);

  println!("Also, the area (method) returns {}", rect.area());
}