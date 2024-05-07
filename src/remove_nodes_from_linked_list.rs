// # 2487 Remove Nodes from Linked List
// note: `daily` for 5/6/24

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

// Note: refer to Monotonic Stack

// first take:
pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack: Vec<Box<ListNode>> = Vec::new();
    let mut curr_node = head;

    while let Some(mut node) = curr_node {
        while !stack.is_empty() && node.val > stack.last().unwrap().val {
            stack.pop();
        }

        curr_node = node.next.take();
        stack.push(node);
    }

    while let Some(mut node) = stack.pop() {
        node.next = curr_node;
        curr_node = Some(node);
    }

    curr_node
}

//
//
//

// Reverse + Remove (using `max`)
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr_node) = (None, head);

    while let Some(mut node) = curr_node {
        curr_node = node.next;
        node.next = prev.take();
        prev = Some(node);
    }

    prev
}

pub fn _remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // for LC:
    // let head = Solution::reverse_list(head);

    let (mut prev, mut curr_node) = (None, head);
    let mut curr_max = i32::MIN;

    while let Some(mut node) = curr_node {
        if node.val > curr_max {
            curr_max = node.val;
        }

        curr_node = node.next;

        if node.val < curr_max {
            continue;
        }

        node.next = prev.take();
        prev = Some(node);
    }

    prev
}
