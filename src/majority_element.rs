use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counter: HashMap<i32, i32> = HashMap::new();

        for n in nums.iter() {
            if let Some((&key, &val)) = counter.get_key_value(n) {
                counter.insert(key, val + 1);
            } else {
                counter.insert(*n, 1);
            }
        }

        counter.into_iter().max_by_key(|x| x.1).unwrap().0
    }
}
