// # 141 Linked List Cycle
// note: `daily` for 3/6/24

// Given Python solution here (no Rust provided by LC):

// def hasCycle(self, head: Optional[ListNode]) -> bool:
//     slow, fast = head, head

//     while fast and fast.next:
//         slow = slow.next
//         fast = fast.next.next

//         if slow == fast:
//             return True

//     return False

// Need smart pointer (ref counter) to `share` ownership - need `cell` to access and modify `Node`s (specifically for creating LL cycle - point one back at another)
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    pub val: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32, next: Option<Rc<RefCell<Node>>>) -> Self {
        Node { val, next }
    }

    pub fn default(val: i32) -> Self {
        Node { val, next: None }
    }
}

// two pointer (`tortoise + hare` algo - Floyd)
pub fn has_cycle(head: Option<Rc<RefCell<Node>>>) -> bool {
    // confirm non empty list
    if let Some(head) = head {
        // init pointers
        let (mut slow, mut fast) = (Rc::clone(&head), Rc::clone(&head));
        // as long as `fast.next` as well as `fast.next.next` (two steps forward)
        // -- exits in the event of "no cycle" (`fast` next `is_none()`) -- cycle would `loop` infinitely (without logic ctrl flow's ptr check)
        while fast.borrow().next.is_some()
            && fast.borrow().next.as_ref().unwrap().borrow().next.is_some()
        {
            // NOTE: need 'temp' variables due to ownership/borrowing: can't borrow `slow/fast` both mutably and immutably in the same scope

            // slow incr by 1
            let next_slow = Rc::clone(&slow.borrow().next.as_ref().unwrap());
            slow = next_slow;

            // fast incr by 2
            let next_fast = Rc::clone(
                &fast
                    .borrow()
                    .next
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .next
                    .as_ref()
                    .unwrap(),
            );
            fast = next_fast;

            // since `Node` values could match, we instead match their pointer ref's for a cycle
            if slow.as_ptr() == fast.as_ptr() {
                // `fast` will always catch back up to `slow` over time if the list cycles
                return true;
            }
        }
    }

    // no cycle detected
    false
}
