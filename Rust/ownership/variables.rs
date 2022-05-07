fn main() {
  /*
    Ownership rules,
    1. Each value has a owner.
    2. Every value has only one owner at a time.
    3. When owner goes out-of-scope then, memory is freed.
  */

  let stack_s1 = "Hello"; // This is a sting literal (stored in stack) (immutable)

  let mut heap_s1 = String::from("Hello"); // This is a String type (stored in heap)
  heap_s1.push_str(", Praveen");
  /*
    stack_s1 -> "Hello"
    heap_s1 -> ptr + len + capacity
  */

  println!("{}", heap_s1);

  let heap_s2 = heap_s1; // This is called `move`

  // Here, rust consider `heap_s1` is not valid (to avoid double free error) and heap_s1 will cause error.
  // println!("{}", heap_s1);

  let heap_s3 = heap_s2.clone(); // This is called `copy`

  println!("{}, {}", heap_s2, heap_s3);
 
} // Here, the owners goes out of scope -> rust calls the `drop` function.