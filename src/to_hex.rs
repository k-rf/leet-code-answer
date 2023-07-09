struct Solution {}

impl Solution {
    pub fn to_hex(mut num: i32) -> String {
        let mut bit: Vec<_> = Vec::new();

        for _ in 0..32 {
            bit.push(num & 1);
            num >>= 1;
        }

        let mut ans = String::new();
        for i in (0..32).step_by(4) {
            ans.insert_str(
                0,
                Self::map(bit[i] * 1 + bit[i + 1] * 2 + bit[i + 2] * 4 + bit[i + 3] * 8).as_str(),
            );
        }

        let ans = ans.trim_start_matches('0').to_string();

        return if ans == "" { "0".to_string() } else { ans };
    }

    fn map(num: i32) -> String {
        match num {
            10 => 'a'.to_string(),
            11 => 'b'.to_string(),
            12 => 'c'.to_string(),
            13 => 'd'.to_string(),
            14 => 'e'.to_string(),
            15 => 'f'.to_string(),
            _ => num.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::to_hex(26), "1a")
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::to_hex(-1), "ffffffff")
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::to_hex(0), "0")
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::to_hex(16), "10")
    }
}
