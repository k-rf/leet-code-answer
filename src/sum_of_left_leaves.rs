use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::TreeNode;

struct Solution {}

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let node = root.unwrap();
        let node = node.borrow();

        return Self::scan(node.left.clone(), true) + Self::scan(node.right.clone(), false);
    }

    fn scan(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();

            if Self::is_leaf(&node) && is_left {
                return node.val;
            } else {
                return Self::scan(node.left.clone(), true) + Self::scan(node.right.clone(), false);
            }
        }

        0
    }

    fn is_leaf(node: &Ref<'_, TreeNode>) -> bool {
        match (node.left.clone(), node.right.clone()) {
            (None, None) => true,
            _ => false,
        }
    }
}
