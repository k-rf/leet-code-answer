use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::TreeNode;

struct Solution;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut flattened = Self::flatten(root, 1);
        flattened.sort_by_key(|&(_, x)| x);

        *flattened
            .iter()
            .fold(
                HashMap::<i32, i32>::new(),
                |mut p: HashMap<i32, i32>, (v, l)| {
                    if let Some(val) = p.get(l) {
                        p.insert(*l, val + v);
                    } else {
                        p.insert(*l, *v);
                    }
                    p
                },
            )
            .iter()
            .max_by_key(|&(_, &x)| x)
            .unwrap()
            .0
    }

    fn flatten(node: Option<Rc<RefCell<TreeNode>>>, level: i32) -> Vec<(i32, i32)> {
        if let Some(node) = node {
            let node = node.borrow();

            return [
                vec![(node.val, level)],
                Self::flatten(node.left.clone(), level + 1),
                Self::flatten(node.right.clone(), level + 1),
            ]
            .concat();
        }

        vec![]
    }
}
