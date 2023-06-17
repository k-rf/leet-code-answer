use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        *nums
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut p, c| {
                if let Some(v) = p.get_mut(c) {
                    *v += 1;
                } else {
                    p.insert(*c, 1);
                }

                p
            })
            .iter()
            .find(|&(_, &y)| y == 1)
            .unwrap()
            .0
    }
}
