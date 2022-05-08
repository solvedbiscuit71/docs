use std::io;

fn main() {
  let mut num = String::new();
  io::stdin()
    .read_line(&mut num)
    .expect("input failed");

  let num: i32 = num.trim().parse().expect("please enter a valid number."); 
  // Here, num is an number from the user.

  const SECRET: i32 = 13;
  if num >= SECRET {
    println!("number is greater or equal to SECRET");
  } else {
    println!("number is lesser than SECRET");
  }
}