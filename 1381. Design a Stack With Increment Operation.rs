pub struct CustomStack {
    data: Vec<i32>,
    max_size: usize,
}
impl CustomStack {
    pub fn new(max_size: i32) -> Self {
        CustomStack {
            data: Vec::with_capacity(max_size as usize),
            max_size: max_size as usize,
        }
    }
    pub fn push(&mut self, x: i32) {
        if self.data.len() < self.max_size {
            self.data.push(x);
        }
    }
    pub fn pop(&mut self) -> i32 {
        self.data.pop().unwrap_or(-1)
    }
    pub fn increment(&mut self, k: i32, val: i32) {
        let k = k.min(self.data.len() as i32) as usize;
        for num in self.data.iter_mut().take(k) {
            *num += val;
        }
    }
}
