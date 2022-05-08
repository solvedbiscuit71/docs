/* 
  This is an working example ./string_slice.rs

  Here, we will get an reference of first_word.
  and it will be valid as long as it's String is valid.
*/
use std::io;

fn first_word(sentence: &String) -> &str {
  for (i, item) in sentence.bytes().enumerate() {
    if item == b' ' {
      return &sentence[0..i];
    }
  }
  &sentence[..]
}

fn main() {
  let mut sentence = String::new();
  io::stdin()
    .read_line(&mut sentence)
    .expect("input failed");

  let word = first_word(&sentence);

  println!("First word is '{}'", word);
}