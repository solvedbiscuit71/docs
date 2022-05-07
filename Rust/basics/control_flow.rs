fn main() {
  let number: i32 = 5;

  if number > 0 {
    println!("Number is greater");
  } else if number < 0 {
    println!("Number is lesser");
  } else {
    println!("Number is 0");
  }

  // Here, if number { will cause type error.
  let result: bool = if number != 0 { true } else { false };

  // since, if is an expression we can use in let.
  println!("{}",result);
}