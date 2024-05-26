// # 2816 Double a Number Represented as a Linked List
// note: `daily` for 5/7/24

// reference: Add Two Numbers II -- https://leetcode.com/problems/add-two-numbers-ii/description/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode { val: 0, next: head };
    let mut curr = &mut dummy;

    while let Some(node) = &mut curr.next {
        curr.val = curr.val * 2;

        if curr.val >= 10 {
            curr.val -= 10;
        }

        if node.val >= 5 {
            curr.val += 1;
        }

        curr = node;
    }

    // if dummy.val == 0 {
    //     return dummy.next.take();
    // } else {
    //     return Some(Box::new(dummy));
    // }

    match dummy.val {
        0 => dummy.next.take(),
        _ => Some(Box::new(dummy)),
    }
}

// convoluted, failing borrow/ownership rules
// pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     // let (mut prev, mut curr) = (Some(Box::new(ListNode { val: 0, next: head })), head); // `prev` set to '0' as 'prefix' (for doubles >= 5)

//     let mut dummy = ListNode { val: 0, next: head };
//     let mut curr = &mut dummy;

//     while let Some(node) = &mut curr.next {
//         if node.val >= 5 {
//             node.val = (node.val * 2) % 10;
//             // prev.as_mut().unwrap().val += 1;
//             dummy.val += 1;
//         }

//         node.val = node.val * 2;
//         // curr = node.next.take();
//         // prev = curr;
//         curr = node;
//     }

//     if dummy.val == 0 {
//         return dummy.next.take();
//     } else {
//         return Some(Box::new(dummy));
//     }
//     // prev
// }
