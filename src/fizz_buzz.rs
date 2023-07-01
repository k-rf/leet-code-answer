struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans = vec![];

        for i in 1..(n + 1) {
            match (i % 3, i % 5) {
                (0, 0) => ans.push(String::from("FizzBuzz")),
                (0, _) => ans.push(String::from("Fizz")),
                (_, 0) => ans.push(String::from("Buzz")),
                (_, _) => ans.push(i.to_string()),
            }
        }

        ans
    }
}
