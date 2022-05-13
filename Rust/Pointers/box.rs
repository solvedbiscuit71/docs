/*
   Single Linked List:

   In C++
       struct Node {
           int data,
           Node* next
       };
*/

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn set_next(&mut self, next: Node<T>) {
        self.next = Some(Box::new(next));
    }
}

fn main() {
    let mut head = Node {
        data: 1,
        next: None,
    };

    head.set_next(Node {
        data: 2,
        next: None,
    });

    println!("{:?}", head);
}
