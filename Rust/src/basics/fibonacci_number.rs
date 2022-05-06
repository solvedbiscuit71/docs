use std::io;

fn nth_fibo(mut num: usize) -> usize {
  let mut a: usize = 0;
  let mut _b: usize = 1;

  return loop {
    (a,_b) = (_b, a + _b);
    num = num - 1;

    if num == 0 {
      break a;
    }
  };
}

fn main() {
  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("something went wrong");

  let input: usize = input.trim().parse().expect("input was not a number");
  println!("The {}th fibonacci number is {}", input, nth_fibo(input));
}