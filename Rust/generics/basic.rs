fn main() {
  let v = vec![52, 98, 100, 4, 8];
  println!("The largest in {:?} is {}", v, largest_item(&v));

  let v1 = vec![1.0, 3.14, 9.5, 5.5, 8.7];
  println!("With generics:");
  println!("The largest in {:?} is {}", v, largest_item_generics(&v));
  println!("The largest in {:?} is {}", v1, largest_item_generics(&v1));
}

// If we want to work with an vector with floating points ?
// then, we have to duplicate!!
fn largest_item(v: &Vec<u32>) -> &u32 {
  let mut max = &v[0];
  for item in v {
    if max < item {
      max = item
    }
  }
  max
}

/*
  Here, we define a generic <T> which an PartialOrd Trait bounded to it.
  NOTE: generics don't create types it defines relationship between types.
*/

// NOTE: here, instead of Vec<T> you can use [T]
fn largest_item_generics<T: PartialOrd>(v: &[T]) -> &T {
  let mut max = &v[0];
  for item in v {
    if max < item {
      max = item
    }
  }
  max
}
