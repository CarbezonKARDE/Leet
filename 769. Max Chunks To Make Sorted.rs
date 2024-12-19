impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut max   = 0;
        let mut count = 0;
        for (i, num) in (0..).zip(arr) {
            max = max.max(num);
            if max == i {
                count += 1;
            }
        }
        count
    }
}
