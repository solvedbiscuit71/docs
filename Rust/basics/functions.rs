use std::io;

/*
  fn function_name(para: type, para1: type,...) -> return_type {
    <Body>
  }

  NOTE: line 18
  which is an expression, i.e which returns an value.
  also note that there is no ';' semi-colon at the end.
*/
fn get_number() -> i32 {
  let mut input = String::new();
  println!("Enter an integer (32-bit): ");

  io::stdin()
    .read_line(&mut input)
    .expect("input failed");
  
  input.trim().parse().expect("please enter a number")
}

/*
  Here, is_zero(number) <=> let num: i32 = number;
  since, number has the <Copy> trait implemented it will be a copy of it.
*/
fn is_zero(num: i32) -> bool {
  if num == 0 {
    true
  } else {
    false
  }
}

// Here, mut num: i32 <=> let mut num: i32 = number;
fn count_till_zero(mut num: i32) {
  loop {
    if num == 0 {
      break;
    }

    print!("{} ", num);
    num = num - 1;
  }
  println!();
}

fn main() {
  let number: i32 = get_number();
  println!("This '{}' is {}", number, if is_zero(number) {"zero"} else {"not a zero."});

  if !is_zero(number) {
    count_till_zero(number);
  }
}