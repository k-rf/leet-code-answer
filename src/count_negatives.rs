struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans: i32 = 0;

        let row_size = grid.len();
        for i in 0..row_size {
            let col_size = grid[i].len();
            for j in 0..col_size {
                if grid[i][j] < 0 {
                    ans += (col_size - j) as i32;
                    break;
                }
            }
        }

        ans
    }
}
