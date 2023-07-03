use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }

        let s_chars = s.chars().collect::<Vec<_>>();
        let goal_chars = goal.chars().collect::<Vec<_>>();

        if s_chars == goal_chars {
            let mut char_set = HashSet::new();
            for &c in s_chars.iter() {
                if !char_set.insert(c) {
                    return true;
                }
            }
            false
        } else {
            let mut pairs = Vec::new();
            for (&a, &b) in s_chars.iter().zip(goal_chars.iter()) {
                if a != b {
                    pairs.push((a, b));
                }
            }

            pairs.len() == 2 && pairs[0].0 == pairs[1].1 && pairs[0].1 == pairs[1].0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::buddy_strings("ab".to_string(), "ba".to_string()),
            true
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::buddy_strings("ab".to_string(), "ab".to_string()),
            false
        )
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::buddy_strings("aa".to_string(), "aa".to_string()),
            true
        )
    }

    #[test]
    fn case4() {
        assert_eq!(
            Solution::buddy_strings("abab".to_string(), "abab".to_string()),
            true
        )
    }

    #[test]
    fn case5() {
        assert_eq!(
            Solution::buddy_strings("ab".to_string(), "babbb".to_string()),
            false
        )
    }
}
