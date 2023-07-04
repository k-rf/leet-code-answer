struct Solution {}

impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in (0..nums.len() - 1).step_by(3) {
            if nums[i] != nums[i + 1] {
                return nums[i];
            }
        }

        *nums.last().unwrap()
    }
}
