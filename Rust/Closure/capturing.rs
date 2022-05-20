use std::mem::drop;

fn main() {
    // by reference (Fn)
    let color = "green".to_string();
    let print = || println!("color: `{}`", color);
    print();

    // by mutable reference (FnMut)
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("New count: {}", count)
    };

    inc();
    println!("{}", count);

    // by value (FnOnce)
    let num = Box::new(5);
    let consume = || {
        println!("{:?}", num);
        drop(num); // Here, drop(T) so value moved!
    };

    consume();
}
