// # 1171 Remove Zero Sum Consecutive Nodes from Linked List
// note: `daily` for 3/12/24

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::collections::HashMap;

pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // ptr. for iteration + for use with edge case re: `head` modification
    let dummy = Some(Box::new(ListNode { val: 0, next: head }));
    // stores `prefix_sum` as key -> `last_node` (ListNode) as val
    let mut last_sum_node: HashMap<i32, &Box<ListNode>> = HashMap::new();
    // cumulative through list iteration: re vals for hashmap (at each 'step' in the list)
    let mut prefix_sum = 0;
    let mut curr_node = dummy.as_ref();

    // first pass: create `last_sum_node`
    while let Some(node) = curr_node {
        // val sum'd so far from lists' `ListNode`s
        prefix_sum += node.val;
        last_sum_node.insert(prefix_sum, node); // refers to `last_node` for val later (the 'up to' node re: removal)
        curr_node = node.next.as_ref();
    }

    let mut dummy_return = Some(Box::new(ListNode::new(0)));
    // pointer from `dummy_return` to 'build' new list from
    let mut curr_node = dummy_return.as_mut();
    // reset - used to calculate 'where' to remove sublist from (current pointer to the `last_node` from `get()` method)
    prefix_sum = 0;

    while let Some(node) = curr_node {
        prefix_sum += node.val;
        // matched 'sum' found -> `curr_node` to `last_node` sums to 0 as sublist
        if let Some(last_node) = last_sum_node.get(&prefix_sum) {
            // set currently pointed at `node`'s `next` to the `last_node` in found sublist's next (ie. remove sublist)
            node.next = last_node.next.clone();
        }
        curr_node = node.next.as_mut();
    }

    dummy_return.unwrap().next
}
