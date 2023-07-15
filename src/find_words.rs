use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let rows: Vec<HashSet<char>> = vec![
            "qwertyuiop".chars().collect(),
            "asdfghjkl".chars().collect(),
            "zxcvbnm".chars().collect(),
        ];

        words
            .into_iter()
            .filter(|word| {
                let c: HashSet<char> = word.to_lowercase().chars().collect();
                rows.iter().any(|row| c.is_subset(row))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::find_words(vec![
                "Hello".to_string(),
                "Alaska".to_string(),
                "Dad".to_string(),
                "Peace".to_string()
            ]),
            vec!["Alaska".to_string(), "Dad".to_string()]
        );
    }

    #[test]
    fn case2() {
        let expected: Vec<String> = vec![];
        assert_eq!(Solution::find_words(vec!["omk".to_string()]), expected);
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::find_words(vec!["asdf".to_string(), "sfd".to_string()]),
            vec!["asdf".to_string(), "sfd".to_string()]
        )
    }
}
