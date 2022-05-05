fn len_ownership(s: String) -> (String, usize) {
  let length = s.len();
  (s, length)
}

fn len_reference(s: &String) -> usize {
  s.len()
}

fn main() {
  let s1 = String::from("Compiler");

  // Here, shadowing of s1 occurs
  let (s1, len1) = len_ownership(s1);

  println!("'{}' has size {}", s1, len1);
  /*
    Instead of passing the ownership around,
    we can let the function borrow (reference) the value.
  */

  let len2 = len_reference(&s1);

  println!("len2 = {}", len2);
} // Here, the memory of s1 is freed.

/*
  Rules:
  -> at anytime, we can any number of immutable reference or only one mutable reference.
  -> reference must always be valid.
*/