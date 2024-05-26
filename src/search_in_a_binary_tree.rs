// # 700 Search in a Binary Tree

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::{cell::RefCell, rc::Rc};

// `take` vs. `clone`
pub fn search_bst_take(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(root_node) => {
            let mut node = root_node.borrow_mut();

            match node.val {
                node_val if node_val > val => search_bst_take(node.left.take(), val),
                node_val if node_val < val => search_bst_take(node.right.take(), val),
                _ => Some(Rc::clone(&root_node)),
            }
        }
        None => None,
    }
}

pub fn search_bst_clone(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(root_node) => {
            let node = root_node.borrow();

            match node.val {
                node_val if node_val > val => search_bst_clone(node.left.clone(), val),
                node_val if node_val < val => search_bst_clone(node.right.clone(), val),
                _ => Some(Rc::clone(&root_node)),
            }
        }
        None => None,
    }
}

// consolidated  (`clone`):
pub fn search_bst_shortened(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) if node.borrow().val < val => {
            search_bst_shortened(node.borrow().right.clone(), val)
        }
        Some(node) if node.borrow().val > val => {
            search_bst_shortened(node.borrow().left.clone(), val)
        }
        Some(node) => Some(node),
        None => None,
    }
}

// less calls to `borrow`:
pub fn search_bst_less_borrow(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let node_ref = node.borrow();

            match node_ref.val {
                ref_val if ref_val < val => search_bst_less_borrow(node_ref.right.clone(), val),
                ref_val if ref_val > val => search_bst_less_borrow(node_ref.left.clone(), val),
                // _ => Some(node.clone()),
                _ => Some(Rc::clone(&node)),
                // needs the clone due to move out of 'node' + ref still existing in scope (`node_ref`)
            }
        }
        None => None,
    }
}

// using `Ordering`
use std::cmp::Ordering;

pub fn search_bst_ordering(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let node_ref = node.borrow();

            match node_ref.val.cmp(&val) {
                Ordering::Equal => Some(Rc::clone(&node)),
                Ordering::Less => search_bst_ordering(node_ref.right.clone(), val),
                Ordering::Greater => search_bst_ordering(node_ref.left.clone(), val),
            }
        }
        None => None,
    }
}

// using `if let` with `or else` (lazy eval) for recursive step:
pub fn search_bst_if_or_else(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let node_ref = node.borrow();

        if node_ref.val == val {
            return Some(node.clone());
        } else {
            search_bst_if_or_else(node_ref.left.clone(), val)
                .or_else(|| search_bst_if_or_else(node_ref.right.clone(), val))
        }
    } else {
        None
    }
}
