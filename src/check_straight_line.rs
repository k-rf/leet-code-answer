struct Solution;

impl Solution {
    pub fn check_straight_line(c: Vec<Vec<i32>>) -> bool {
        for i in 2..c.len() {
            if (c[i - 1][1] - c[i - 2][1]) * (c[i][0] - c[i - 1][0])
                != (c[i][1] - c[i - 1][1]) * (c[i - 1][0] - c[i - 2][0])
            {
                return false;
            }
        }

        true
    }
}
