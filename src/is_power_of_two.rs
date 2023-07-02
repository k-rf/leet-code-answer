struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 0 {
            return false;
        }

        let n = n as i64;
        n & (n - 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::is_power_of_two::Solution;

    #[test]
    fn case1() {
        assert_eq!(Solution::is_power_of_two(1), true)
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::is_power_of_two(16), true)
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::is_power_of_two(3), false)
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::is_power_of_two(-1), false)
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::is_power_of_two(i32::MIN), false)
    }

    #[test]
    fn case6() {
        assert_eq!(Solution::is_power_of_two(0), false)
    }
}
