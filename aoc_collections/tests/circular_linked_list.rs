extern crate aoc_common;

use aoc_common::collections::CircularLinkedList;
use core::borrow::Borrow;

#[test]
fn test_push() {
    let mut circular_list = CircularLinkedList::<usize>::new();

    circular_list.push_front(2);
    circular_list.push_front(1);
    circular_list.push_back(3);

    assert_contains(circular_list, vec![1, 2, 3].into_iter());
}

fn assert_contains<T, I>(mut circular_list: CircularLinkedList<T>, mut expected: I)
where
    T: PartialEq + std::fmt::Debug,
    I: Iterator<Item = T>,
{
    while let Some(expected_value) = expected.next() {
        assert_eq!(circular_list.pop_front(), Some(expected_value));
    }

    assert_eq!(circular_list.len(), 0);
}

#[test]
fn test_from_iterator() {
    let circular_list = vec![1, 2, 3]
        .into_iter()
        .collect::<CircularLinkedList<usize>>();

    assert_contains(circular_list, vec![1, 2, 3].into_iter());
}

#[test]
fn test_iterator() {
    let circular_list = vec![1, 2, 3]
        .into_iter()
        .collect::<CircularLinkedList<usize>>();

    let vector_collection = circular_list
        .iter()
        .map(|v| v.clone())
        .collect::<Vec<usize>>();

    assert_eq!(vector_collection, vec![1, 2, 3]);
}

#[test]
fn test_cursor() {
    let mut circular_list = vec![1, 2, 3]
        .into_iter()
        .collect::<CircularLinkedList<usize>>();

    let mut cursor = circular_list.cursor_mut();

    assert_eq!(cursor.current(), Some(&mut 1));
    assert_eq!(cursor.current(), Some(&mut 1));

    cursor.move_next();
    assert_eq!(cursor.current(), Some(&mut 2));

    cursor.move_next();
    assert_eq!(cursor.current(), Some(&mut 3));

    cursor.move_next();
    assert_eq!(cursor.current(), Some(&mut 1));

    cursor.move_back();
    assert_eq!(cursor.current(), Some(&mut 3));

    cursor.move_back();
    assert_eq!(cursor.current(), Some(&mut 2));
}

#[test]
fn test_cursor_insert() {
    let mut circular_list = vec![1].into_iter().collect::<CircularLinkedList<usize>>();

    let mut cursor = circular_list.cursor_mut();

    cursor.insert(0);
    cursor.move_next();

    cursor.insert(2);
    cursor.insert(3);

    cursor.move_next();

    assert_eq!(cursor.current(), Some(&mut 3));

    let vector: Vec<usize> = circular_list.iter().map(|v| v.clone()).collect();

    assert_eq!(vector, vec![1, 0, 3, 2])
}

#[test]
fn test_cursor_remove() {
    let mut circular_list = vec![1, 2, 3]
        .into_iter()
        .collect::<CircularLinkedList<usize>>();

    let mut cursor = circular_list.cursor_mut();

    cursor.move_next();

    assert_eq!(cursor.current(), Some(&mut 2));
    assert_eq!(cursor.remove(), Some(2));

    assert_eq!(cursor.current(), Some(&mut 3));

    assert_contains(circular_list, vec![1, 3].into_iter());
}
