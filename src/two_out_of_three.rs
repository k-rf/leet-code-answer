use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_out_of_three(
        mut nums1: Vec<i32>,
        mut nums2: Vec<i32>,
        mut nums3: Vec<i32>,
    ) -> Vec<i32> {
        nums1.sort();
        nums1.dedup();

        nums2.sort();
        nums2.dedup();

        nums3.sort();
        nums3.dedup();

        let mut hash = HashMap::new();

        for num in [nums1, nums2, nums3].concat() {
            hash.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }

        hash.iter()
            .filter_map(|(&x, &y)| {
                if 2 <= y {
                    return Some(x);
                } else {
                    return None;
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut actual = Solution::two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]);
        let mut expect = vec![3, 2];

        actual.sort();
        expect.sort();

        assert_eq!(actual, expect)
    }

    #[test]
    fn case2() {
        let mut actual = Solution::two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2]);
        let mut expect = vec![2, 3, 1];

        actual.sort();
        expect.sort();

        assert_eq!(actual, expect)
    }

    #[test]
    fn case3() {
        let mut actual = Solution::two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5]);
        let mut expect = vec![];

        actual.sort();
        expect.sort();

        assert_eq!(actual, expect)
    }
}
