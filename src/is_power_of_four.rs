struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n != 0 && (f32::log2(n as f32) as i32) % 2 == 0 && n as i64 & (n as i64 - 1) == 0
    }
}
