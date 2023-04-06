//template for Node class
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
        }
    }
}

//template for LinkedList class
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn add_to_front(&mut self, data: T) {
        let new_node = Node::new(data);
        let prev_node = self.head.take();
        new_node.next = prev_node;
        self.head = Some(Box::new(new_node));
    }
}
