struct Solution;

impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut ans = 0;

        for _ in 0..(a.max(b.max(c)) as f32).log2() as i32 {
            match (a & 1, b & 1, c & 1) {
                (0, 0, 0) | (1, _, 1) | (_, 1, 1) => (),
                (1, 1, 0) => ans += 2,
                _ => ans += 1,
            }

            a >>= 1;
            b >>= 1;
            c >>= 1;
        }

        ans
    }
}
