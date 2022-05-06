use std::io;

/*
  if let is an syntax sugar for match,
  when we want to check specfic case and leave other cases unchecked.
*/
fn main() {
  let mut num = String::new();
  io::stdin()
    .read_line(&mut num)
    .expect("input failed");

  // let res: Option<i32> = match num.trim().parse() {
  //   Ok(n) => Some(n),
  //   Err(_) => None
  // };

  let res: Option<i32>;
  if let Ok(n) = num.trim().parse() {
    res = Some(n);
  } else {
    res = None;
  }

  println!("Your result is {:?}", res);
}