#[derive(Debug)]
enum IpArr {
  V4(String),
  V6(String),
}

impl IpArr {
  fn get(&self) -> &IpArr {
    self
  }
}

fn main() {
  let home_ip = IpArr::V4(String::from("127.0.0.1"));
  dbg!(home_ip.get());

  /*
    built-in enum Option<T>
    1. Some(T)
    2. None

    Here, in rust we don't have a concept of `null`
  */

  let some_number = Some(5);
  let none: Option<u32> = None; // Here, we need to type inference.
}