mod test;
/*  Definition for singly-linked list. */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from(collection: &Vec<i32>) -> ListNode {
        let collection = collection.iter().rev().collect::<Vec<&i32>>();

        let mut list = ListNode {
            val: *collection[0],
            next: None,
        };

        for i in 1..collection.len() {
            let current = collection[i];

            let new_node = ListNode {
                val: *current,
                next: Some(Box::new(list)),
            };

            list = new_node;
        }

        list
    }
}

/// It takes a vector of integers and returns a linked list
///
/// Arguments:
///
/// * `collection`: &Vec<i32>
///
/// Returns:
///
/// A ListNode
pub fn to_list_node(collection: &Vec<i32>) -> ListNode {
    let collection = collection.iter().rev().collect::<Vec<&i32>>();

    let mut list = ListNode {
        val: *collection[0],
        next: None,
    };

    for i in 1..collection.len() {
        let current = collection[i];

        let new_node = ListNode {
            val: *current,
            next: Some(Box::new(list)),
        };

        list = new_node;
    }

    list
}

/// It takes a pointer to a list node, and returns a vector of the values in the list
///
/// Arguments:
///
/// * `list`: &Option<Box<ListNode>>
///
/// Returns:
///
/// A vector of i32
fn get_value(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut pointer = list;

    let mut vector = Vec::new();

    loop {
        if let Some(item) = pointer {
            vector.push(item.val);
            pointer = &item.next;
        } else {
            break;
        }
    }

    vector
}

/// I convert the linked lists to vectors, then I convert the vectors to strings, then I convert the
/// strings to integers, then I add the integers, then I convert the integer to a string, then I convert
/// the string to a vector, then I convert the vector to a linked list
///
/// Arguments:
///
/// * `l1`: Option<Box<ListNode>>
/// * `l2`: Option<Box<ListNode>>
///
/// Returns:
///
/// A list node
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let value_1 = get_value(&l1);
    let value_2 = get_value(&l2);

    let value_1 = value_1
        .iter()
        .rev()
        .map(|int| int.to_string())
        .collect::<String>();

    let value_2 = value_2
        .iter()
        .rev()
        .map(|int| int.to_string())
        .collect::<String>();

    let sum = value_1.parse::<u64>().expect("xd") + value_2.parse::<u64>().expect("XD");

    let array = sum
        .to_string()
        .chars()
        .map(|int| int.to_string().parse::<i32>().expect("XD2"))
        .rev()
        .collect::<Vec<i32>>();

    Some(Box::new(to_list_node(&array)))
}

fn main() {}
