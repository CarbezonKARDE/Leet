impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut last = -1;
        for bitlike in nums.chunk_by(|a, b| a.count_ones() == b.count_ones()) {
            let front = *bitlike.iter().min().unwrap();
            if front < last { return false; }
            last = *bitlike.iter().max().unwrap();
        }
        true
    }
}
