/*
  Derive the 'Debug' triat for use {:?} in println! macro
*/

#[derive(Debug)]
struct Rect {
  width: u32,
  height: u32,
}

impl Rect {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  /*
    Here, we don't pass the self (as it's call on Struct itself)
    :: is used instead of .
  */
  fn with_size(size: u32) -> Rect {
    Rect {
      width: size,
      height: size,
    }
  }

  fn with_dimension(width: u32, height: u32) -> Rect {
    Rect {
      width,
      height,
    }
  }
}

fn main() {
  let square = Rect::with_size(15);
  let rect = Rect::with_dimension(10, 5);

  println!("Square {:?} has area of {}", square, square.area());
  println!("Rectangle {:?} has area of {}", rect, rect.area());
}