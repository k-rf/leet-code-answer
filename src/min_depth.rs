use std::{cell::RefCell, rc::Rc};

use crate::TreeNode;

struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(_) = root {
            Self::traverse(root, 0)
        } else {
            0
        }
    }

    fn traverse(node: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        if Self::is_leaf(&node) {
            return depth + 1;
        }

        if let Some(node) = node {
            let node = node.borrow();

            std::cmp::min(
                Self::traverse(node.left.clone(), depth + 1),
                Self::traverse(node.right.clone(), depth + 1),
            )
        } else {
            i32::MAX
        }
    }

    fn is_leaf(node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = node {
            let node = node.borrow();

            match (node.left.clone(), node.right.clone()) {
                (None, None) => true,
                _ => false,
            }
        } else {
            false
        }
    }
}
