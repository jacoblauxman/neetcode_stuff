#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// two pointer (sorta) `tortoise and hare` (Floyd)
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut slow, mut fast) = (&head, &head);

    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }

    slow.clone()
}

// ALT: using a counter instead:

pub fn middle_node_with_counter(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut counter = 0;
    let mut curr_node = &head;

    while let Some(node) = curr_node {
        counter += 1;
        curr_node = &node.next;
    }

    for _ in 0..counter / 2 {
        head = head.unwrap().next;
    }
    head
}
