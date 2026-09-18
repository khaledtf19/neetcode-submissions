struct MinStack {
    val: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self { val: vec![] }
    }

    pub fn push(&mut self, val: i32) {
        self.val.push(val);
    }

    pub fn pop(&mut self) {
        self.val.pop().unwrap();
    }

    pub fn top(&self) -> i32 {
        let Some(v) = self.val.last() else {
            return 0;
        };
        v.clone()
    }

    pub fn get_min(&self) -> i32 {
        let mut min = self.val[0].clone();
        for &v in self.val.iter() {
            if v < min {
                min = v
            }
        }
        min
    }
}
