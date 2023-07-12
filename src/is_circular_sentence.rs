struct Solution {}

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let splitted: Vec<&str> = sentence.split_whitespace().collect();

        let mut last_word = splitted.last().unwrap().chars().last().unwrap();

        for s in splitted {
            let f = s.chars().collect::<Vec<char>>()[0];
            let l = s.chars().last().unwrap();

            if last_word == f {
                last_word = l;
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::is_circular_sentence("leetcode exercises sound delightful".to_string(),),
            true
        );
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::is_circular_sentence("eetcode".to_string()), true);
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::is_circular_sentence("Leetcode is cool".to_string()),
            false
        );
    }
}
