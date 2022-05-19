pub struct Conc {
    data: i32,
    next: Box<Link>,
}

pub enum Link {
    Node(Conc),
    Nil,
}

impl Conc {
    fn new(data: i32) -> Self {
        Self {
            data,
            next: Box::new(Link::Nil),
        }
    }
}

use Link::{Nil, Node};

struct List {
    head: Box<Link>,
    len: usize,
}

impl List {
    fn new() -> Self {
        Self {
            head: Box::new(Nil),
            len: 0,
        }
    }

    fn push(&mut self, data: i32) {
        let head = &mut self.head;
        match **(head) {
            Node(ref mut head_node) => {
                let mut current = head_node;
                loop {
                    match *(current.next) {
                        Node(ref mut node) => current = node,
                        Nil => break,
                    }
                }
                current.next = Box::new(Node(Conc::new(data)));
            }
            Nil => {
                **(head) = Node(Conc::new(data));
            }
        }
        self.len += 1;
    }

    fn pop(&mut self) {
        let head = &mut self.head;

        // Updating the len if head != Nil (or head == Node(_))
        if let Node(_) = **(head) {
            self.len -= 1;
        }

        match **(head) {
            Node(ref mut head_node) => match *(head_node.next) {
                Nil => {
                    **(head) = Nil;
                    return;
                }
                _ => {
                    let mut current = head_node;
                    loop {
                        // Checking if current->next->next = Nil
                        match *(current.next) {
                            Node(ref node) => {
                                if let Nil = *(node.next) {
                                    break;
                                }
                            }
                            Nil => (),
                        }

                        // if not, then move current to next
                        if let Node(ref mut node) = *(current.next) {
                            current = node;
                        }
                    }
                    *(current.next) = Nil;
                }
            },
            Nil => (),
        }
    }

    fn show(&self) {
        let head = &self.head;
        match **(head) {
            Node(ref head_node) => {
                let mut current = head_node;
                loop {
                    match *(current.next) {
                        Node(ref node) => {
                            print!("{} -> ", current.data);
                            current = node;
                        }
                        Nil => {
                            println!("{} -> Null", current.data);
                            break;
                        }
                    }
                }
            }
            Nil => println!("Null"),
        }
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    list.show();
    println!("Length is {}", list.len);

    list.pop();
    list.show();
    println!("Length is {}", list.len);
}
