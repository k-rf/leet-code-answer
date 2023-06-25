struct Solution;

impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        nums.sort();

        let before = nums.len();

        nums.dedup();

        let after = nums.len();

        before != after
    }
}
