use std::convert::From;

struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    let num2: Number = 30.into();

    /*
     * NOTE: from() and into() are interchangable!
     */

    println!("My number is {}", num.value);
    println!("My number is {}", num2.value);
}
