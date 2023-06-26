struct OrderedStream {
    index: i32,
    stream: Vec<Option<String>>,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            index: 1,
            stream: vec![None; n as usize],
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.stream[(id_key - 1) as usize] = Some(value);

        let mut result = Vec::new();
        if id_key == self.index {
            while let Some(Some(s)) = self.stream.get((self.index - 1) as usize) {
                result.push(s.clone());
                self.index += 1;
            }
        }

        result
    }
}
