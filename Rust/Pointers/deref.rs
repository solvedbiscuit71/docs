use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x) // Here, we storing the data in stack (but, it doesn't matter)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
    For, deref to work the Type should have implemented the Deref Trait.
    Box<T> has already implemented it.
 */
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, *y); // *y <=> *(y.deref()) //
}
