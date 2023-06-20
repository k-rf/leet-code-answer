struct Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = vec![-1; nums.len()];
        let mut memo = nums[0..(2 * k).min(nums.len() as i32) as usize]
            .iter()
            .fold(0_i128, |acc, cur| acc + *cur as i128);

        for i in k..(nums.len() as i32 - k) {
            let begin = (i - k) as usize;
            let end = (i + k) as usize;

            memo += nums[end] as i128;
            ans[i as usize] = (memo / ((k * 2) + 1) as i128) as i32;
            memo -= nums[begin] as i128;
        }

        ans
    }
}
