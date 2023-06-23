struct Solution;

impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len() as i32;

        nums.sort();

        let mut dup = 0;
        for x in nums.windows(2) {
            if x[0] == x[1] {
                dup = x[0];
            }
        }
        nums.dedup();

        for (a, b) in nums.iter().enumerate() {
            let a = (a + 1) as i32;

            if a != *b {
                return vec![dup, a];
            }
        }

        if nums[0] == 1 {
            return vec![dup, size];
        } else {
            return vec![dup, nums[0] - 1];
        }
    }
}
