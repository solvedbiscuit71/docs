enum Possible {
  Correct,
  Wrong,
}
/*
  match control flow syntax,
  helps to match the enums with the correct variant.
*/

fn check_possible(p: Possible) -> Option<u32> {
  match p {
    Possible::Correct => {
      println!("It's correct");
      return Some(10);
    },
    Possible::Wrong => {
      println!("It's wrong");
      return None;
    }
  }
}

fn main() {
  let p = Possible::Wrong;
  let opt: Option<u32> = check_possible(p);
  match opt {
    Some(value) => println!("Value associated is {}", value),
    None => println!("it's null")
  }

  // An example of match control flow
  let mut v: Vec<i32> = vec![1,2,3,-1];
  match v.pop() {
    Some(value) => println!("Value poped is {}", value),
    None => println!("Vector is empty")
  }

  /*
  NOTE:
  - it's not just limited to enums
  - match expression are evaluation in order.
  */

  let x: u8 = 5;
  match x {
    0 => println!("it's zero"),
    _ => println!("it's not zero")
  }
  // Here, _ means all other cases of u8.
}