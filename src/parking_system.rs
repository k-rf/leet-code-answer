struct ParkingSystem {
    big: Capacity,
    medium: Capacity,
    small: Capacity,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            big: Capacity::new(big),
            medium: Capacity::new(medium),
            small: Capacity::new(small),
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => self.big.add(1),
            2 => self.medium.add(1),
            3 => self.small.add(1),
            _ => false,
        }
    }
}

struct Capacity {
    max: i32,
    val: i32,
}

impl Capacity {
    fn new(max: i32) -> Self {
        Capacity { max, val: 0 }
    }

    fn add(&mut self, val: i32) -> bool {
        let res = self.val + val;

        if self.max < res {
            false
        } else {
            self.val = res;
            true
        }
    }
}
