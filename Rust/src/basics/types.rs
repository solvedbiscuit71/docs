fn main() {
  /*
    Primitive data types:
    1. i8,i16,..,i128,isize (signed)
    2. u8,u16,..,u128,usize (unsigned)
    3. f32,f64 (floats)
    4. char
  */
  let _x: i32 = -45;
  let _y: f64 = 0.05;

  let _c: char = 'c';

  /*
    Compound types:
    1. tuple (heterogeneous) -> accessing by "tuple.0"
    2. array [homogeneous] -> accessing by "array[0]"
  */

  let t: (u32, f32, char) = (25, 25.05, 't');
  let a: [i32; 5] = [3,4,6,28,98];

  let a1 = [0; 5];

  println!("Tuples last element was {}", t.2);
  println!("Array second element was {}", a[1]);
  println!("{:?}",a1)
}