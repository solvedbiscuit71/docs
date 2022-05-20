#![allow(dead_code)]

#[derive(Debug)]
struct Unit;

struct Pair(i32, i32);

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let pair = Pair(35, 55);
    println!("{}, {}", pair.0, pair.1);

    let point = Point { x: 5, y: -10 };
    println!("({}, {})", point.x, point.y);

    let some = Unit;
    println!("{:?}", some);
}
