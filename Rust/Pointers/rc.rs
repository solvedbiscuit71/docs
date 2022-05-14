use std::rc::Rc;

#[derive(Debug)]
struct Info {
    name: String,
    age: u8,
    email: String
}

impl Info {
    fn new(name: String, age: u8, email: String) -> Info {
        Info { name, age, email }
    }
}

/*
    Rc<T> allows you to share data between multiple parts of your program for reading only. 

    But, interior mutability pattern and the RefCell<T> type 
    you can use in conjunction with an Rc<T> to work with this immutability restriction.
*/
fn main() {
    let p1 = Rc::new(Info::new("praveen".to_string(), 18, "ppraveen98841@gmail.com".to_string()));
    println!("{:?}",p1);

    let p2 = Rc::clone(&p1);
    println!("{:?}",p2);

    println!("Ref Count: {}", Rc::strong_count(&p1));
}