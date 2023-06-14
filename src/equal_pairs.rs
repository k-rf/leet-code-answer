struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ans = 0;

        for i in 0..n {
            for j in 0..n {
                let mut c = 0;
                for k in 0..n {
                    c += (grid[i][k] == grid[k][j]) as i32;
                }
                ans += (c == n as i32) as i32
            }
        }

        ans
    }
}
