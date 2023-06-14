use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

struct Solution;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut flattened = Self::flatten(root);
        flattened.sort();

        flattened
            .windows(2)
            .map(|x| (x[0] - x[1]).abs())
            .min()
            .unwrap()
    }

    fn flatten(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = node {
            let node = node.borrow();

            return [
                vec![node.val],
                Self::flatten(node.left.clone()),
                Self::flatten(node.right.clone()),
            ]
            .concat();
        }

        vec![]
    }
}
