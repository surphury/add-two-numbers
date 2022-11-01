#[cfg(test)]
use crate::{add_two_numbers, get_value, ListNode};

#[test]
fn test_1() {
    let list1 = Some(Box::new(ListNode::from(&vec![2, 4, 3])));
    let list2 = Some(Box::new(ListNode::from(&vec![5, 6, 4])));

    let result = add_two_numbers(list1, list2);
    assert_eq!(get_value(&result), vec![7, 0, 8]);
}

#[test]
fn test_2() {
    let list1 = Some(Box::new(ListNode::from(&vec![0])));
    let list2 = Some(Box::new(ListNode::from(&vec![0])));

    let result = add_two_numbers(list1, list2);
    assert_eq!(get_value(&result), vec![0]);
}

#[test]
fn test_3() {
    let list1 = Some(Box::new(ListNode::from(&vec![9, 9, 9, 9, 9, 9, 9])));
    let list2 = Some(Box::new(ListNode::from(&vec![9, 9, 9, 9])));

    let result = add_two_numbers(list1, list2);
    assert_eq!(get_value(&result), vec![8, 9, 9, 9, 0, 0, 0, 1]);
}

#[test]
fn test_4() {
    let list1 = Some(Box::new(ListNode::from(&vec![
        1, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    ])));
    let list2 = Some(Box::new(ListNode::from(&vec![9])));

    let result = add_two_numbers(list1, list2);
    assert_eq!(get_value(&result), vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
}
