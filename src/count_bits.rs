struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; (n + 1) as usize];

        for i in 0..(n + 1) {
            let mut j = i;

            while 0 < j {
                ans[i as usize] += j & 1;
                j >>= 1;
            }
        }

        ans
    }
}
