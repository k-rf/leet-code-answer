struct Solution {}

impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        1 < n && 3_i32.pow((i32::MAX as f64).log(3.0) as u32) % n == 0
    }
}
