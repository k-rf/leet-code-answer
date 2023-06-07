struct Solution;

impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut ans = 0;

        let mut max = a.max(b.max(c));
        let mut len = 0;

        while 0 < max {
            max >>= 1;
            len += 1;
        }

        for _ in 0..len {
            let (x, y, z) = (a & 1, b & 1, c & 1);

            match (x, y, z) {
                (1, 1, 1) => (),
                (1, 0, 1) => (),
                (0, 1, 1) => (),
                (0, 0, 1) => ans += 1,

                (1, 1, 0) => ans += 2,
                (1, 0, 0) => ans += 1,
                (0, 1, 0) => ans += 1,
                (0, 0, 0) => (),
                _ => (),
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }

        ans
    }
}
