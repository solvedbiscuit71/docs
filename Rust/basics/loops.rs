/*
  There are 3 types of loops in Rust.
  1. loop -> forever; manual stopping required
  2. while -> conditional;
  3. for -> conditional;
*/

fn main() {
  let mut count = 3;

  println!("Counting down...");
  let status = loop {
    println!("{}", count);

    count = count - 1;
    if count == 0 {
      println!("Launch!!!");
      break 404; // return value from an loop.
    }
  };

  println!("Status Code: {}", status);

  // while loop (conditional looping)

  count = 3;
  println!("Relaunch.. Counting down...");

  while count > 0 {
    println!("{}", count);
    count = count - 1;
  }
  println!("Launch!!!");


  // for loop (iterating over array)

  let secert_code = [4,2,6,8,0];
  for digit in secert_code {
    print!("{}.", digit);
  }
  println!("")
}