struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut ans = 0;

        for i in 0..prices.len() {
            min = prices[i].min(min);

            if min < prices[i] {
                continue;
            }

            for j in (i + 1)..prices.len() {
                if prices[j] <= prices[i] {
                    break;
                }

                ans = (prices[j] - prices[i]).max(ans);
            }
        }

        ans
    }
}
