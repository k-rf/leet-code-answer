use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            Self::is_same_tree_other(
                root.borrow_mut().left.clone(),
                root.borrow_mut().right.clone(),
            )
        } else {
            false
        }
    }

    pub fn is_same_tree_other(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let (p, q) = (p.borrow_mut(), q.borrow_mut());

                Self::is_same_tree_other(p.left.clone(), q.right.clone())
                    && Self::is_same_tree_other(p.right.clone(), q.left.clone())
                    && p.val == q.val
            }
            (None, None) => true,
            _ => false,
        }
    }
}
