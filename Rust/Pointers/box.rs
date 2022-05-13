/*
   Box<T>
       - instead of storing the value in stack it stores in heap.
       - useful, if we don't know the size at compile time.
*/
use Link::*;

#[derive(Debug)]
enum Link {
    Node(Box<NodeType>),
    Nil,
}

#[derive(Debug)]
struct NodeType {
    data: usize,
    next: Link,
}

impl NodeType {
    fn new(data: usize) -> NodeType {
        NodeType { data, next: Nil }
    }
}

fn set_next(head: &mut Link, node: NodeType) {
    let new_link = Node(Box::new(node));
    match head {
        Node(head_node) => {
            head_node.next = new_link;
        }
        Nil => *(head) = new_link,
    }
}

fn main() {
    let mut head = Nil;
    set_next(&mut head, NodeType::new(1));
    set_next(&mut head, NodeType::new(2));

    println!("{:?}", head);

    /*
    Check: ../Example/linked_list.rs
    for the full implementation of linked list.
     */
}
