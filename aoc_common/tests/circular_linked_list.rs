extern crate aoc_common;

use aoc_common::collections::CircularLinkedList;

#[test]
fn test_push() {
    let mut circular_list = CircularLinkedList::<usize>::new();

    circular_list.push_front(2);
    circular_list.push_front(1);
    circular_list.push_back(3);

    assert_contains(circular_list, vec![1, 2, 3].into_iter());
}

fn assert_contains<T, I>(mut circular_list: CircularLinkedList<T>, mut expected: I)
    where T : PartialEq + std::fmt::Debug,
    I : Iterator<Item = T> {

    while let Some(expected_value) = expected.next() {
        assert_eq!(circular_list.pop_front(), Some(expected_value));
    }

    assert_eq!(circular_list.len(), 0);
}

#[test]
fn test_from_iterator() {
    let circular_list = vec![1, 2, 3].into_iter().collect::<CircularLinkedList<usize>>();

    assert_contains(circular_list, vec![1, 2, 3].into_iter());
}

#[test]
fn test_iterator() {
    let circular_list = vec![1, 2, 3].into_iter().collect::<CircularLinkedList<usize>>();

    let vector_collection = circular_list.iter().map(|v| v.clone()).collect::<Vec<usize>>();

    assert_eq!(vector_collection, vec![1, 2, 3]);
}