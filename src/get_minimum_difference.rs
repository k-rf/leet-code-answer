use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;

struct Solution;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut flattened = Self::flatten(root);
        flattened.sort();

        let mut ans = i32::MAX;

        for i in 1..flattened.len() {
            ans = ans.min(flattened[i] - flattened[i - 1]);
        }

        ans
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
