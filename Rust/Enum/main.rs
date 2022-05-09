use std::io;

enum Number {
  Zero,
  Negative,
  Positive
}

fn get_number() -> i32 {
  let mut s = String::new();
  io::stdin()
    .read_line(&mut s)
    .expect("input failed");
    
  s.trim().parse().expect("Input is not a number")
}

fn main() {
  let x = get_number();
  let status: Number;

  if x > 0 {
    status = Number::Positive;
  } else if x < 0 {
    status = Number::Negative;
  } else {
    status = Number::Zero;
  }

  // Now, status can either be one of the possible variant.
  // We can use an match statement to define what to do on each variant!
  match status {
    Number::Positive => println!("it's a positive number"),
    Number::Negative => println!("it's a negative number"),
    Number::Zero => println!("it's zero")
  }
}