use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        if let Some(root) = root {
            ans.append(&mut Self::inorder_traversal(root.borrow().left.clone()));
            ans.push(root.borrow().val);
            ans.append(&mut Self::inorder_traversal(root.borrow().right.clone()));
        }

        ans
    }
}
