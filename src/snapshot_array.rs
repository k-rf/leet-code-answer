use std::collections::LinkedList;

struct SnapshotArray {
    gen: i32,
    val: Vec<LinkedList<Option<i32>>>,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            gen: 0,
            val: vec![LinkedList::from([None]); length as usize],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let snap = &mut self.val[index as usize];

        if let Some(last) = snap.iter_mut().last() {
            *last = Some(val);
        } else {
            snap.push_back(Some(val));
        }
    }

    fn snap(&mut self) -> i32 {
        for elm in self.val.iter_mut() {
            if !(*elm).is_empty() {
                (*elm).push_back(*(*elm).iter().last().unwrap());
            }
        }

        let gen = self.gen;
        self.gen += 1;
        return gen;
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let mut target = self.val[index as usize].iter();
        let mut ans = target.next();

        for _ in 0..snap_id {
            ans = target.next();
        }

        ans.unwrap().unwrap_or(0)
    }
}
