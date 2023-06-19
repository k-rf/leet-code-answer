struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut total = 0;

        for i in gain {
            total += i;

            ans = ans.max(total);
        }

        ans
    }
}
