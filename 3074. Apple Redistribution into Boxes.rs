impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut capacity = capacity;
        capacity.sort();
        capacity.reverse();
        let mut sum :i32 = apple.iter().sum();
        let mut i = 0usize;
        while i < capacity.len() && sum > 0 {
            sum -= capacity[i];
            i += 1;
        }
        i as i32
    }
}
