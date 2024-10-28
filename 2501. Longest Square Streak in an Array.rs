use std::collections::HashSet;
impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut unique_nums: Vec<i64> = nums.into_iter()
            .map(|x| x as i64)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        unique_nums.sort_unstable();
        let num_set: HashSet<i64> = unique_nums.iter().cloned().collect();
        let mut max_length = 0;
        for &num in &unique_nums {
            let mut length = 0;
            let mut current = num;
            while num_set.contains(&current) {
                length += 1;
                if current > (1i64 << 31) {
                    break;
                }
                current *= current;
            }
            if length > 1 {
                max_length = max_length.max(length);
            }
        }
        if max_length > 1 {
            max_length
        } else {
            -1
        }
    }
}
