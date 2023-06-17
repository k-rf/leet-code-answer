struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut ans = vec![0; (row_index + 1) as usize];

        for i in 0..(row_index + 1) {
            ans[i as usize] = Self::comb(row_index, i)
        }

        ans
    }

    fn comb(n: i32, k: i32) -> i32 {
        let mut a: u128 = 1;
        let mut b: u128 = 1;

        for i in 0..(n - k).min(k) {
            a *= (n - i) as u128;
            b *= (i + 1) as u128;
        }

        (a / b) as i32
    }
}
