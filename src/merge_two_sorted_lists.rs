// # 21 Merge Two Sorted Lists

// from LC
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

// note for LC: need to insert `Self::merge_two_lists()` for recursive fn calls
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        // Returns `Some` list # dependent on which of list in tuple are `None`
        (Some(list), None) | (None, Some(list)) => Some(list),
        // merge logic (both `Some`)
        (Some(mut list1), Some(mut list2)) => {
            // comparison of curr ListNodes' `val`s
            // lesser `val` takes precedence in call stack and its `next` = recursive call to the given lesser's next + the other list
            if list1.val < list2.val {
                list1.next = merge_two_lists(list1.next, Some(list2));
                Some(list1)
            } else {
                list2.next = merge_two_lists(Some(list1), list2.next);
                Some(list2)
            }
        }
        _ => None, // case of both lists being empty on first call
    }
}
