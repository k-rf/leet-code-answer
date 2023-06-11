use std::collections::HashMap;

#[derive(Debug)]
struct SnapshotArray {
    gen: i32,
    snap: Vec<HashMap<i32, i32>>,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            gen: 0,
            snap: vec![HashMap::new(); length as usize],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.snap[index as usize].insert(self.gen, val);
    }

    fn snap(&mut self) -> i32 {
        return self.next_gen();
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let target = &self.snap[index as usize];

        if let Some(snap) = target.get(&snap_id) {
            return *snap;
        } else {
            for i in 1..snap_id + 1 {
                if let Some(res) = target.get(&(snap_id - i)) {
                    return *res;
                }
            }
            return 0;
        }
    }

    fn next_gen(&mut self) -> i32 {
        let gen = self.gen;
        self.gen += 1;

        gen
    }
}
