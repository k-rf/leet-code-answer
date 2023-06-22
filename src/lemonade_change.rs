struct Solution;

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut b5, mut b10) = (0, 0);

        for bill in bills {
            match bill {
                5 => {
                    b5 += 1;
                }
                10 => {
                    b10 += 1;
                    b5 -= 1;
                }
                20 => {
                    if 0 < b10 {
                        b10 -= 1;
                        b5 -= 1;
                    } else {
                        b5 -= 3;
                    }
                }
                _ => (),
            }

            if b5 < 0 || b10 < 0 {
                return false;
            }
        }

        true
    }
}
