use std::ptr::NonNull;
use std::iter::FromIterator;

pub struct CircularLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    value: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            prev: None,
            next: None,
        }
    }
}

// private methods
impl<T> CircularLinkedList<T> {
    fn insert_between_head_tail(&mut self, value: T) -> NonNull<Node<T>> {
        let new_node = match self.head {
            None => self.create_first(value),
            Some(mut head) => {
                match self.tail {
                    None => panic!("Invalid state where only head is set but not tail."),
                    Some(mut tail) => {
                        let mut new_node = unsafe {
                            Box::into_raw_non_null(Box::new(Node::new(value)))
                        };

                        unsafe {
                            let mut node = new_node.as_mut();
                            node.next = Some(head);
                            node.prev = Some(tail);
                        }

                        unsafe {
                            head.as_mut().prev = Some(new_node);
                            tail.as_mut().next = Some(new_node);
                        }

                        new_node
                    }
                }
            },
        };
        new_node
    }

    fn create_first(&mut self, value: T) -> NonNull<Node<T>> {
        let mut first_node = unsafe {
            Box::into_raw_non_null(Box::new(Node::new(value)))
        };

        unsafe {
            let mut node = first_node.as_ptr();
            (*node).next = Some(first_node);
            (*node).prev = Some(first_node);
        }

        self.head = Some(first_node);
        self.tail = Some(first_node);

        first_node
    }

    fn push_node_front(&mut self, value: T) {
        let new_node = self.insert_between_head_tail(value);
        self.head = Some(new_node);
        self.len += 1;
    }

    fn push_node_back(&mut self, value: T) {
        let new_node = self.insert_between_head_tail(value);
        self.tail = Some(new_node);
        self.len += 1;
    }

    fn pop_node_front(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            // last element
            if self.len == 1 {
                self.head = None;
                self.tail = None;
            } else {
                let pop_node = node.as_ref();
                self.head = pop_node.next;
                pop_node.prev.unwrap().as_mut().next = pop_node.next;
                pop_node.next.unwrap().as_mut().prev = pop_node.prev;
            }

            self.len -= 1;

            Box::from_raw(node.as_ptr())
        })
    }
}


// public methods
impl<T> CircularLinkedList<T> {
    pub fn new() -> Self {
        CircularLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.push_node_front(value);
    }

    pub fn push_back(&mut self, value: T) {
        self.push_node_back(value);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_node_front().map(|node| node.value)
    }
}

impl<A> FromIterator<A> for CircularLinkedList<A> {
    fn from_iter<T: IntoIterator<Item=A>>(iter: T) -> Self {
        let mut iterator = iter.into_iter();
        let mut result = CircularLinkedList::new();

        while let Some(element) = iterator.next() {
            result.push_back(element);
        }

        result
    }
}

#[cfg(test)]
mod tests {

}