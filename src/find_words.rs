struct Solution {}

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let rows = vec![
            "qwertyuiop".to_string(),
            "asdfghjkl".to_string(),
            "zxcvbnm".to_string(),
        ];

        let mut ans = vec![];
        for word in words.iter() {
            for row in rows.iter() {
                let lower_word = word.to_lowercase();

                let row = row.chars().collect::<Vec<_>>();
                let lower_word = lower_word.chars().collect::<Vec<_>>();

                if let Some(_) = row.iter().find(|&&x| x == lower_word[0]) {
                    let mut skip = false;
                    for c in lower_word.iter() {
                        if !row.contains(c) {
                            skip = true;
                            break;
                        }
                    }
                    if !skip {
                        ans.push(word.clone());
                    }
                }
            }
        }

        ans
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
