struct MyHashSet {
    val: Vec<i32>,
}

impl MyHashSet {
    fn new() -> Self {
        MyHashSet { val: vec![] }
    }

    fn add(&mut self, key: i32) {
        if !self.contains(key) {
            self.val.push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        if let Some(pos) = self.val.iter().position(|&x| x == key) {
            self.val.remove(pos);
        }
    }

    fn contains(&self, key: i32) -> bool {
        if let None = self.val.iter().find(|&&x| x == key) {
            false
        } else {
            true
        }
    }
}
