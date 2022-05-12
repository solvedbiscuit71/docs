/*
  Closure are anonymous function.

  NOTE:
    - no type annotation is required.
    - but, you can add if need!
*/

fn main() {
    let add10 = |num| num + 10;
    let mut num = 5;

    println!("{}", num);
    num = add10(num);
    println!("{}", num);
}
