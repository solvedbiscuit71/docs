fn main() {
  const PI: f32 = 3.14; // By conversion, use UPPERCASE
  /* 
    Here, type inference is done by compiler.
    let -> immutable (can be changed) and auto type inference
    const -> immutable always and manual type inference
  */

  let x = "Praveen";
  let x = x.len();
  /*
    Here, we are redeclaring the same variable x, (shadowing)
    -> type of the variable changes.
    -> no need of 'mut' keyword.
  */

  println!("PI is {}", PI);
  println!("Also, x is {}", x);
}