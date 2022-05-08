fn main() {
  let mut sum_till_5 = 0;

  // Here, 0..=5 creates an iterator starts from 0 to 5 (inclusive)
  for num in 0..=5 {
    sum_till_5 += num;
  }
  println!("Sum of numbers from 0..5 is {}", sum_till_5);

  let mut count = 3;
  while count > 0 {
    print!("{}... ", count);
    count -= 1;
  }
  println!("Start!");

  /*
  loop is an infinite loop which requires manual breaking by, 'break' statement;
  */

  let mut count = 0;
  let return_value = loop {
    if count % 2 == 0 {
      print!("{} ", count);

      count += 1;
      continue;
    } else if count > 10 {
      break count;
    }

    count += 1;
  };

  println!("and Return Value: {}", return_value);
}