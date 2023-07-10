struct Solution {}

impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.partial_cmp(a).unwrap());
        nums.dedup();

        return match nums.len() {
            1 | 2 => nums[0],
            _ => nums[2],
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1)
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::third_max(vec![1, 2]), 2)
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1)
    }
}
