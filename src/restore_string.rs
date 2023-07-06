struct Solution {}

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut tuple: Vec<(char, i32)> = s.chars().zip(indices).collect();
        tuple.sort_by_key(|(_, y)| *y);

        tuple.iter().map(|(x, _)| x).collect()
    }
}
