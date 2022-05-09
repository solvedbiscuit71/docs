/*
  There are some default enum in rust
  1. Option<T>
  2. Result<T,E>

  enum Option {
    Some(T),
    None
  }

  enum Result {
    Ok(T),
    Err(E)
  }
*/

fn main() {
  let mut v = vec![1,2,3,4]; // This is a vector (dynamic array)

  v.push(5);
  println!("Vec: {:?}", v);

  /*
    Here, we can pop the last value and return it.
    but, what if the vector is empty -> then we can't pop anything

    So, there are two possibility
    1. Vector is not empty (Some)
    2. Vector is empty (None)
  */
  
  // v.clear(); // Try clearing the vector.
  match v.pop() {
    Some(n) => println!("Value poped is {}", n),
    None => println!("Vector is empty")
  }
}