fn main() {
  /*
    let {variable_name}: {type} = {value};

    Here,
    - variable_name usually snake_case!
    - type (can be implicity)
    - value (value relavent to that type)

    Note:
    - by default, let defines a immutable variable.
    - to make a mutable variable, add 'mut' keyword.
  */
  let _x: i32 = -45;
  let mut _y: f64 = 0.05;

  let _c = 'c'; // automatically, inferred as 'char'

  /*
    Compound types:
    1. tuple (heterogeneous) -> accessing by "tuple.0"
    2. array [homogeneous] -> accessing by "array[0]"
  */

  let tuple: (u32, f32, char) = (25, 25.05, 't');
  let array: [i32; 5] = [3,4,6,28,98];

  let array_generated = [0; 5];

  // Note: {} -> Display trait & {:?} -> Debug trait
  println!("Tuple: {:?}, Array: {:?}", tuple, array);
  println!("Array (generated): {:?}",array_generated);
}