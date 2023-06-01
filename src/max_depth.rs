use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::traverse(root, 0)
    }

    fn traverse(node: Option<Rc<RefCell<TreeNode>>>, d: i32) -> i32 {
        match node {
            None => d,
            Some(node) => {
                let node = node.borrow();
                std::cmp::max(
                    Self::traverse(node.left.clone(), d + 1),
                    Self::traverse(node.right.clone(), d + 1),
                )
            }
        }
    }
}
