fn add2(x: i32) {
  println!("After adding 2, {} becomes {}", x, x + 2);
}

fn add2_return(x: i32) -> i32 {
  x + 2 // This is a return statement (implicitly)
}

fn main() {
  hello_world();

  let x: i32 = 45;
  add2(x);

  let x = add2_return(x);
  println!("x after modified {}", x);
}

// NOTE: it's defined after main()
fn hello_world() {
  println!("Hello World")
}