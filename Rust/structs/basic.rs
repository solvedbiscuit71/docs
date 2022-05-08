struct Rect {
  width: u32,
  height: u32,
}

fn area(rect: &Rect) -> u32 {
  rect.width * rect.height
}

fn main() {
  let mut rect = Rect {
    width: 10,
    height: 5,
  };

  println!("{}x{} is the dimension", rect.width, rect.height);

  rect.width = 15;
  println!("{}x{} is the dimension", rect.width, rect.height);

  println!("Also, the area is {}", area(&rect));
}