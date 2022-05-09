use std::io;

fn main() {
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("input failed");
  
  let res: Result<i32, _> = input.trim().parse();

  if let Ok(n) = res {
    println!("your number is {}", n);
  } else {
    panic!("input is not a number"); // throw error;
  }

  /*
  match res {
    Ok(n) => println!("your number is {}", n),
    _ => println!("your input is not valid")
  }
  */
}