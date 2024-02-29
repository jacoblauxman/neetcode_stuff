// # 206 Reverse Linked List

// from LC:
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[allow(dead_code)]
impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut curr, mut prev) = (head, None);

    while let Some(mut node) = curr {
        // detach `next` ListNode from current node - holds its reference
        let next = node.next.take();
        // reverse the `next` point of current node "backwards"
        node.next = prev;

        // -- move forward -- //
        // set `prev` to point at our current position in Linked List
        prev = Some(node);
        // set `curr` towards reference saved of original `next` ListNode
        curr = next;
    }

    // return - `prev` is the first item in the reversed LL
    prev
}
