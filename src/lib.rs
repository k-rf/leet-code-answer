use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

mod can_make_arithmetic_progression;
mod check_straight_line;
mod design_hash_set;
mod has_path_sum;
mod inorder_traversal;
mod is_balanced;
mod is_same_tree;
mod is_symmetric;
mod kth_largest;
mod majority_element;
mod max_depth;
mod min_depth;
mod min_flips;
mod parking_system;
mod pascal_triangle;
mod sorted_array_to_bst;
mod top_k_frequent;
