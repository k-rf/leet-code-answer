use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.len() != s.split_whitespace().collect::<Vec<_>>().len() {
            return false;
        }

        let mut ans = true;

        let mut ps: HashMap<_, _> = HashMap::new();
        let mut sp: HashMap<_, _> = HashMap::new();

        for (key, value) in pattern.chars().zip(s.split_whitespace()) {
            ps.entry(key)
                .and_modify(|x| {
                    if *x != value {
                        ans = false;
                    }
                })
                .or_insert(value);

            sp.entry(value)
                .and_modify(|x| {
                    if *x != key {
                        ans = false;
                    }
                })
                .or_insert(key);
        }

        ans
    }
}
