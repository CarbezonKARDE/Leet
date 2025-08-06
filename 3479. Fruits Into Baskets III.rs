struct SegmentTree {
    size: usize,
    data: Vec<i32>,
}
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut tree = SegmentTree::new(&baskets);
        let mut unplaced = 0;
        for fruit in fruits {
            if let Some(idx) = tree.find_first_ge(fruit) {
                tree.update(idx, 0);
            } else {
                unplaced += 1;
            }
        }
        unplaced
    }
}
impl SegmentTree {
    fn new(values: &[i32]) -> Self {
        let size = values.len().next_power_of_two();
        let mut data = vec![0; 2 * size];
        data[size..size + values.len()].copy_from_slice(values);
        for i in (1..size).rev() {
            data[i] = data[2 * i].max(data[2 * i + 1]);
        }
        Self { size, data }
    }
    fn find_first_ge(&self, val: i32) -> Option<usize> {
        let mut pos = 1;
        while pos < self.size {
            pos *= 2;
            if self.data[pos] < val {
                pos += 1;
            }
        }
        if pos >= self.size && self.data[pos] >= val {
            Some(pos - self.size)
        } else {
            None
        }
    }
    fn update(&mut self, idx: usize, val: i32) {
        let mut pos = self.size + idx;
        self.data[pos] = val;
        while pos > 1 {
            self.data[pos / 2] = self.data[pos].max(self.data[pos ^ 1]); 
            pos /= 2;
        }
    }
}
