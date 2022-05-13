use Link::*;

pub enum Link {
    Node(Box<NodeType>),
    Nil,
}

pub struct NodeType {
    data: u8,
    next: Link,
}

impl NodeType {
    fn new(data: u8) -> NodeType {
        NodeType { data, next: Nil }
    }
}

/*
   Pushing new element cases:

   Case: 1
   Nil => Head

   Case: 2
   Head -> Nil => Head -> Node -> Nil

   Case: 3
   Head -> Node -> ... -> Nil => Head -> Node -> ... Node -> Nil
*/
pub fn push(head: &mut Link, data: u8) {
    match head {
        Node(ref mut head) => {
            let mut current = head;
            loop {
                match current.next {
                    Node(ref mut next) => current = next,
                    Nil => {
                        current.next = Node(Box::new(NodeType::new(data)));
                        break;
                    }
                }
            }
        }
        Nil => {
            *(head) = Node(Box::new(NodeType::new(data)));
        }
    }
}

/*
    Popping last element cases:

    Case: 1
    Nil => Nil

    Case: 2
    Head -> Nil => Nil

    Case: 3
    Head -> Node -> Nil => Head -> Nil

    Case: 4
    Head -> Node ... -> Node -> Nil => Head -> Node ... -> Nil
*/
pub fn pop(head: &mut Link) {
    match head {
        Nil => (),
        Node(ref mut head_node) => {
            if let Nil = head_node.next {
                *(head) = Nil;
            } else {
                let mut current = head_node;

                loop {
                    // Check if current->next->next == Nil ??
                    match current.next {
                        Node(ref node) => {
                            if let Nil = node.next {
                                current.next = Nil;
                                break;
                            }
                        }
                        Nil => (),
                    }

                    // If not, then move current = current->next
                    match current.next {
                        Node(ref mut node) => {
                            current = node;
                        }
                        Nil => (),
                    }
                }
            }
        }
    }
}

pub fn show(head: &Link) {
    match head {
        Node(ref head) => {
            let mut current = head;
            loop {
                match current.next {
                    Node(ref node) => {
                        print!("{} -> ", current.data);
                        current = node;
                    }
                    Nil => {
                        println!("{} -> Nil", current.data);
                        break;
                    }
                }
            }
        }
        Nil => println!("List is Empty"),
    }
}

fn main() {
    let mut head = Nil;
    push(&mut head, 1);
    push(&mut head, 2);
    push(&mut head, 3);
    push(&mut head, 4);

    show(&head);
    pop(&mut head);
    show(&head);
}
