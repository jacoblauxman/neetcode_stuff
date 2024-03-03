// # 19 Remove Nth Node From End of List
// note: `daily` for 3/3/24

// Definition for singly-linked list.
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

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // create dummy ListNode for 'off by 1' re: `left` pointer (connect ListNode before AND after target node from end of list via `next` reassignment)
    // let mut dummy = Box::new(ListNode::new(0));
    // dummy.next = head; // given code from LC doesn't allow us to assign `next` in the new() method call

    // NOTE: the above seemed to give a '1ms' runtime (vs '0ms') of initializing as such:
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    // create `left` and `right` or `follower` and `leader` pointers - both 'start' at dummy
    let mut leader = dummy.clone();
    let mut follower = dummy.as_mut(); // `dummy` is a `Box`, need a mutable ref

    // iterate leader `n` times forward to be offset by that distance in list
    for _ in 0..n {
        leader = leader.next.unwrap();
    } // -> able to keep track of how far from end of list `left/follwer` is via `right/leader` hitting end / `None` value (the point of reassignment)

    // iterate forward until `right/leader` hits end of list (until leader's `next` is `None`)
    while leader.next.is_some() {
        leader = leader.next.unwrap();
        follower = follower.next.as_mut().unwrap();
    }

    // reassign `left/follower` so that `next` pointer is to our `removed` ListNode's `next`
    let removed = follower.next.take().unwrap(); // `take` explicitly moves `Some` value out of Option in place for `None` (explicit reassignment?)
    follower.next = removed.next;
    // or this:
    // let removed = follower.next.as_mut().unwrap(); // as_ref() works too, we're not going to mutate `removed`,
    // follower.next = removed.next.clone();

    // return `dummy.next` (the `head` of our LL)
    dummy.next
}
