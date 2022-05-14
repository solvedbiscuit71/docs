use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn greet(name: &str) {
    println!("Hello, {}", name);
}

/*
    Deref Coercion:
    - Compiler automatically dereference to the value into desired value for function & method.

    NOTE: Deref Trait should be implemented!

    Rust will do deref coercion when,
    - From &T to &U when T: Deref<Target=U>
    - From &mut T to &mut U when T: DerefMut<Target=U>
    - From &mut T to &U when T: Deref<Target=U>
 */

fn main() {
    let s = MyBox::new(String::from("Praveen"));
    greet(&s); // &s <=> &(*s)[..] //

    /*
        Here, we are passing &MyBox<String>
        which is deref to
        -> &String (because we implemented Deref) -> &str (std already implemented &String -> &str)

        Here, &MyBox<String> -> &String because MyBox<String>: Deref<Target=String>
     */
}
