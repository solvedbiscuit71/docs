// Here, we are using an generic T in struct
struct Point<T> {
  x: T,
  y: T,
}

/*
  Here, we are implementing an show() method
  But, it's only defined for Point<T> which as
  std::fmt::Display implemented.
*/
impl<T: std::fmt::Display> Point<T> {
  fn show(&self) {
    println!("Point {{ x: {}, y: {} }}", self.x, self.y);
  }
}

impl Point<char> {
  fn special(&self) {
    println!("This is an special kind of Point.")
  }
}

fn main() {
  let p = Point { x: 5, y: 10 };
  p.show();

  let p1 = Point { x: 'a', y: 'b' };
  p1.special(); // NOTE: only p1 has this method defined.
}
