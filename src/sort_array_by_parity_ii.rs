struct Solution {}

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let even: Vec<_> = nums.clone().into_iter().filter(|&x| x % 2 == 0).collect();
        let odd: Vec<_> = nums.into_iter().filter(|&x| x % 2 != 0).collect();

        even.iter()
            .zip(odd.iter())
            .fold(Vec::new(), |acc, (&e, &o)| [acc, vec![e, o]].concat())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
            vec![4, 5, 2, 7]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::sort_array_by_parity_ii(vec![3, 4]), vec![4, 3]);
    }
}
