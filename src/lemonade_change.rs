use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut revenue = HashMap::from([(5, 0), (10, 0)]);

        for bill in bills {
            match bill {
                5 => {
                    revenue.entry(5).and_modify(|c| *c += 1);
                }
                10 => {
                    revenue.entry(10).and_modify(|c| *c += 1);
                    revenue.entry(5).and_modify(|c| *c -= 1);
                }
                20 => {
                    let b10 = revenue.get_mut(&10).unwrap();
                    if 0 < *b10 {
                        *b10 -= 1;
                        revenue.entry(5).and_modify(|c| *c -= 1);
                    } else {
                        revenue.entry(5).and_modify(|c| *c -= 3);
                    }
                }
                _ => (),
            }

            if !revenue.values().all(|&x| 0 <= x) {
                return false;
            }
        }

        true
    }
}
