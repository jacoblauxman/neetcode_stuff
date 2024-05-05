// # 237 Delete Node in a Linked List
// note: `daily` 5/5/24
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)] // due to updated logic below
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn delete_node(node: &mut Option<Box<ListNode>>) {
    if let Some(node) = node {
        *node = node.next.take().unwrap();
    }
}
