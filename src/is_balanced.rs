use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::traverse(root, 0).1
    }

    fn traverse(node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> (i32, bool) {
        if let Some(node) = node {
            let node = node.borrow();

            let l = Self::traverse(node.left.clone(), depth + 1);
            let r = Self::traverse(node.right.clone(), depth + 1);

            return (std::cmp::max(l.0, r.0), (l.0 - r.0).abs() < 2 && l.1 && r.1);
        }

        (depth, true)
    }
}
