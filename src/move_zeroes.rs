struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut c = 0;

        nums.retain(|&x| {
            if x != 0 {
                return true;
            } else {
                c += 1;
                return false;
            }
        });

        for _ in 0..c {
            nums.push(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut value = vec![0, 1, 0, 3, 12];

        Solution::move_zeroes(&mut value);

        assert_eq!(value, vec![1, 3, 12, 0, 0])
    }

    #[test]
    fn case2() {
        let mut value = vec![0];

        Solution::move_zeroes(&mut value);

        assert_eq!(value, vec![0])
    }
}
