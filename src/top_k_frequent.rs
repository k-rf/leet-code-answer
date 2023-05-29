use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            if let Some(v) = res.remove(&num) {
                res.insert(num, v + 1);
            } else {
                res.insert(num, 1);
            }
        }

        let mut res: Vec<(i32, i32)> = res.into_iter().map(|x| x).collect();
        res.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

        res.iter().as_slice()[0..k as usize]
            .iter()
            .map(|(k, _)| *k)
            .collect()
    }
}
