use std::ptr::NonNull;
use std::iter::FromIterator;
use std::fmt::Debug;

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

pub struct Iter<'a, T> {
    list: &'a CircularLinkedList<T>,
    current_node: Option<NonNull<Node<T>>>,
}

impl<'a, T> Iter<'a, T> {
    fn new(list: &'a CircularLinkedList<T>) -> Iter<'a, T> {
        Iter {
            list,
            current_node: list.head,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result: Option<Self::Item> = self.current_node.map(|nn_node| unsafe {
            &(*nn_node.as_ptr()).value
        });

        unsafe {
            self.current_node = self.current_node.and_then(|nn_node| {
                if self.list.tail == Some(nn_node) {
                    None
                } else {
                    nn_node.as_ref().next
                }
            })
        }

        result
    }
}

pub struct CursorMut<'a, T> {
    list: &'a mut CircularLinkedList<T>,
    current_node: Option<NonNull<Node<T>>>
}

// private methods
impl<'a, T> CursorMut<'a, T> {
    fn new(list: &'a mut CircularLinkedList<T>) -> CursorMut<'a, T> {
        let current_node = list.head;
        CursorMut {
            list,
            current_node,
        }
    }

    fn insert_node(&mut self, value: T) {
        match self.current_node {
            None => self.list.push_front(value),
            Some(mut next) => unsafe {
                let mut prev = match next.as_ref().prev {
                    None => panic!("Invalid state where node has no predecessor."),
                    Some(mut prev) => prev
                };

                let mut new_node = Node::new(value);

                new_node.next = Some(next);
                new_node.prev = Some(prev);

                let mut new_node = Box::into_raw_non_null(Box::new(new_node));

                next.as_mut().prev = Some(new_node);
                prev.as_mut().next = Some(new_node);

                if self.list.head == Some(next) {
                    self.list.head = Some(new_node);
                }

                self.list.len += 1;
            }
        }
    }

    fn remove_node(&mut self) -> Option<Box<Node<T>>> {
        self.current_node.map(|nn_node| unsafe {
            let node = nn_node.as_ref();
            let mut next = match node.next {
                None => panic!("Invalid state where node has no successor."),
                Some(next) => next
            };

            let mut prev = match node.prev {
                None => panic!("Invalid state where node has no predecessor."),
                Some(prev) => prev
            };

            if prev == next {
                self.current_node = None;
                self.list.head = None;
                self.list.tail = None;
            } else {
                let prev_node = prev.as_mut();
                let next_node = next.as_mut();

                prev_node.next = node.next;
                next_node.prev = node.prev;

                if self.list.head == Some(nn_node) {
                    self.list.head = Some(next)
                } else if self.list.tail == Some(nn_node) {
                    self.list.tail = Some(prev);
                }

                self.current_node = Some(next);
            }

            self.list.len -= 1;

            Box::from_raw(nn_node.as_ptr())
        })
    }
}

// public methods
impl<'a, T> CursorMut<'a, T> {
    pub fn current(&mut self) -> Option<&'a mut T> {
        self.current_node.map(|nn_node| unsafe {
            &mut (*nn_node.as_ptr()).value
        })
    }

    pub fn move_next(&mut self) {
        self.current_node = self.current_node.and_then(|nn_node| unsafe {
            nn_node.as_ref().next
        })
    }

    pub fn move_back(&mut self) {
        self.current_node = self.current_node.and_then(|nn_node| unsafe {
            nn_node.as_ref().prev
        })
    }

    pub fn insert(&mut self, value: T) {
        self.insert_node(value);
    }

    pub fn remove(&mut self) -> Option<T> {
        self.remove_node().map(|node| node.value)
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
                let mut prev_node = pop_node.prev.unwrap();
                prev_node.as_mut().next = pop_node.next;
                let mut next_node = pop_node.next.unwrap();
                next_node.as_mut().prev = pop_node.prev;
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

    pub fn len(&self) -> usize {
        self.len
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

    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {
        CursorMut::new(self)
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