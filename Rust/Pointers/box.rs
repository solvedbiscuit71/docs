fn main() {
    // Box::new() -> similar to make_unique() in C++ [it creates an unique ptr]
    let num: Box<i32> = Box::new(5);
    let mut num2 = num; // -> value moved to num2

    println!("{}", num2);
    num2 = Box::new(10);

    println!("After reassignment: {}", num2);
}
