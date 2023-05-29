struct KthLargest {
    k: usize,
    sorted: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut nums = nums;
        nums.sort();

        KthLargest {
            k: k as usize,
            sorted: nums,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        println!("{:?}", self.sorted);
        let size = self.sorted.len();
        for i in 0..size + 1 {
            if i == size {
                self.sorted.insert(i, val);
                break;
            }
            if val < self.sorted[i] {
                self.sorted.insert(i, val);
                break;
            }
        }

        self.sorted[size - self.k + 1]
    }
}
