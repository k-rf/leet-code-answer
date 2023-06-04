struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();

        for n in 0..num_rows {
            match n {
                0 => {
                    res.append(&mut vec![vec![1]]);
                }
                1 => {
                    res.append(&mut vec![vec![1, 1]]);
                }
                _ => {
                    let mut row: Vec<i32> = Vec::new();
                    row.append(&mut vec![1]);
                    for i in 0..(n - 1) {
                        row.append(&mut vec![
                            res[(n - 1) as usize][i as usize]
                                + res[(n - 1) as usize][(i + 1) as usize],
                        ]);
                    }
                    row.append(&mut vec![1]);
                    res.append(&mut vec![row]);
                }
            }
        }

        res
    }
}
