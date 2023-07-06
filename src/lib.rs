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

mod buddy_strings;
mod can_make_arithmetic_progression;
mod check_straight_line;
mod contains_duplicate;
mod count_bits;
mod count_negatives;
mod design_hash_set;
mod equal_pairs;
mod find_error_nums;
mod fizz_buzz;
mod get_averages;
mod get_minimum_difference;
mod get_row;
mod has_path_sum;
mod inorder_traversal;
mod is_balanced;
mod is_palindrome;
mod is_power_of_four;
mod is_power_of_three;
mod is_power_of_two;
mod is_same_tree;
mod is_symmetric;
mod kth_largest;
mod largest_altitude;
mod lemonade_change;
mod longest_subarray;
mod majority_element;
mod max_depth;
mod max_level_sum;
mod max_profit;
mod max_value;
mod min_depth;
mod min_flips;
mod next_greatest_letter;
mod ordered_stream;
mod parking_system;
mod pascal_triangle;
mod restore_string;
mod single_number;
mod single_number2;
mod snapshot_array;
mod sorted_array_to_bst;
mod sum_of_left_leaves;
mod summary_ranges;
mod top_k_frequent;
mod word_pattern;
