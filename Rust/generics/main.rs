/*
  Here, _Point can only contain i32 integer
  but what if we want another point with f32 floating points.
*/
struct _Point {
  x: i32,
  y: i32,
}

/*
  That's where Generics come to play,
  Generics -> relationship between types.

  Here, i am saying that type of x & y are same and it can be anything.
*/
struct Point<T> {
  x: T,
  y: T
}

fn main() {
  let p1 = Point {
    x: 1.0,
    y: 1.5
  }; // p1: Point<f32>

  let p2 = Point {
    x: 2,
    y: 5
  }; // p2: Point<i32>

  let p3 = Point {
    x: 35,
    y: 2.5
  }; // p3 is not valid
}
