struct Solution;

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let mut d = max_sum - n;
        let mut ans = 0;

        while 0 <= d {
            let r = (index + ans + 1).min(n) - (index + 1);
            let l = index - (index - ans).max(0);

            if r == n - index - 1 && l == index {
                ans += d / (r + l + 1) + 1;
                break;
            }
            d = d - r - l - 1;

            ans += 1;
        }

        ans
    }
}
