struct Solution {}

impl Solution {
    pub fn longest_subarray(mut nums: Vec<i32>) -> i32 {
        if nums.iter().all(|x| *x == 1) {
            return nums.len() as i32 - 1;
        }

        let mut block: Vec<i32> = vec![0];
        let mut count = 0;

        for i in 0..nums.len() {
            if nums[i] == 1 {
                block[count] += 1;
            } else {
                count += 1;
                block.push(0);
            }
        }

        block.windows(2).map(|x| x.iter().sum()).max().unwrap()
    }
}
