struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .chars()
            .filter(|c| char::is_ascii_alphanumeric(c))
            .map(|c| c.to_ascii_lowercase())
            .collect();

        let len = chars.len();
        for i in 0..(len / 2) {
            if chars[i] != chars[len - i - 1] {
                return false;
            }
        }

        true
    }
}
