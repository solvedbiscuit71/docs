/*
  Lifetimes are defined by the scope of the variable.

  lifetime elision rules:
  - each parameter that is a reference gets its own lifetime parameter

  - if there is exactly one input lifetime parameter,
    that lifetime is assigned to all output lifetime parameters

  - if there are multiple input lifetime parameters,
    but one of them is &self or &mut self because this is a method,
    then lifetime of self is assigned to all output lifetime parameters.

  Generic lifetime annotation:
    'a, 'b, 'c,... it's a conversion.
    'static -> special meaning lives through a entire program (string literals).
*/

fn largest_string_of<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  if s1.len() > s2.len() {
    s1
  } else {
    s2
  }
}

fn main() {
  let name1 = String::from("Praveen");
  let name2 = "John";

  println!(
    "The largest string is {}",
    largest_string_of(name1.as_str(), name2)
  );
}
