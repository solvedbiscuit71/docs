fn apply<T: Fn()>(func: T) {
    func();
}

fn return_closure() -> impl Fn() {
    let info = "Calling return_closure()".to_owned();
    move || println!("{}", info)
}

fn main() {
    let x = 5;
    let show_x = || println!("{}", x);

    // Here, we are passing closure as an input parameter!!
    apply(show_x);

    // Here, we are returning a closure with it's all value MOVED.
    let closure = return_closure();
    closure();
}
