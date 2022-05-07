fn main() {
  let x = 5; // This is immutable variable.
  let mut y = 6;
  y = 10; // Note: we can't change to another data type.

  const PI: f32 = 3.14;

  /* 
    Here, type inference is done by compiler.
    let -> immutable (can be changed) and auto type inference
    const -> immutable always and manual type inference
  */

  let x = x + 1;
  /*
    This is called "Shadowing",
    -> we are declaring a variable with the same name.
    -> which is used to overwrite the type inference.
  */

  let spaces = "   ";
  let spaces = spaces.len();
}