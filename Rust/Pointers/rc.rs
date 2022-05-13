use std::rc::Rc;

/*
   Eventhough, Box<T> stores the value in heap,
   it can still have one owner!

   That's where Rc<T> comes (Reference Counting)

   1. Rc::clone(&var) -> let you create another owner.
*/

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Rc<Node>>,
    prev: Option<Rc<Node>>,
}

impl Node {
    fn append(&mut self, node: Rc<Node>) {
        self.next = Some(Rc::clone(&node));
    }
}

fn main() {
    let mut head = Rc::new(Node {
        data: 1,
        next: None,
        prev: None,
    });

    let tail = Rc::new(Node {
        data: 2,
        next: None,
        prev: None,
    });

    head.append(tail); // This will cause error!! -> Deref (so we use RefCell<T>)
    println!("{:?}", head);
}
