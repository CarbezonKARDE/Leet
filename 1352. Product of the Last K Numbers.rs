pub struct ProductOfNumbers {
    nums: Vec<i32>,
}
impl ProductOfNumbers {
    pub fn new() -> Self {
        Self { nums: Vec::new() }
    }
    pub fn add(&mut self, num: i32) {
        if num == 0 {
            self.nums.clear();
        } else {
            self.nums.push(self.nums.last().copied().unwrap_or(1) * num);
        }
    }
    pub fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let n = self.nums.len();
        match k.cmp(&n) {
            std::cmp::Ordering::Greater => 0,
            std::cmp::Ordering::Equal => self.nums[n - 1],
            std::cmp::Ordering::Less => self.nums[n - 1] / self.nums[n - k - 1],
        }
    }
}
