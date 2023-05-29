use std::cell::RefCell;
use std::rc::Rc;

use crate::{Solution, TreeNode};

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match nums.len() {
            0 => None,
            _ => {
                let middle = nums.len() / 2;

                Some(Rc::new(RefCell::new(TreeNode {
                    val: nums[middle],
                    left: Self::sorted_array_to_bst(nums[..middle].to_vec()),
                    right: Self::sorted_array_to_bst(nums[middle + 1..].to_vec()),
                })))
            }
        }
    }
}
