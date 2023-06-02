use std::cell::RefCell;
use std::rc::Rc;

use crate::TreeNode;
struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(root) = root {
            let root = root.borrow();
            match (root.left.clone(), root.right.clone()) {
                (None, None) => target_sum - root.val == 0,
                _ => {
                    Self::has_path_sum(root.left.clone(), target_sum - root.val)
                        || Self::has_path_sum(root.right.clone(), target_sum - root.val)
                }
            }
        } else {
            false
        }
    }
}
