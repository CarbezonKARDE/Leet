impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let (min_index, max_index, _, _) = nums.iter().enumerate().fold(
            (0, 0, i32::MAX, i32::MIN),
            |(mut min_index, mut max_index, mut min, mut max), (i, &n)| {
                if n < min {
                    min = n;
                    min_index = i;
                }
                if n > max {
                    max = n;
                    max_index = i;
                }
                (min_index, max_index, min, max)
            },
        );
        let left = min_index.min(max_index);
        let right = nums.len() - 1 - min_index.max(max_index);
        let middle = min_index.max(max_index) - min_index.min(max_index) - 1;
        (left + right).min(left + middle).min(middle + right) as i32 + 2
    }
}
