struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut s = 0;

        let mut ans: Vec<_> = vec![];

        let len = nums.len();

        for (i, j) in (0..len).zip(1..len) {
            if nums[j] - nums[i] != 1 {
                if nums[s] == nums[i] {
                    ans.push(format!("{}", nums[s]));
                } else {
                    ans.push(format!("{}->{}", nums[s], nums[i]));
                }
                s = j;
            }
        }

        match nums[s..].len() {
            0 => (),
            1 => ans.push(format!("{}", nums[s])),
            _ => ans.push(format!("{}->{}", nums[s], nums.last().unwrap())),
        }

        ans
    }
}
