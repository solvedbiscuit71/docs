fn first_word(s1: &String) -> &str {
  let bytes = s1.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s1[0..i];
    }
  }
  &s1[..]
}

fn main() {
  let s1 = String::from("Hello World");
  let word = first_word(&s1);

  println!("First word is '{}'", word);
}