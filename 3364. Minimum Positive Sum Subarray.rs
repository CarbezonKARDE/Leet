impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        (l..=r)
            .flat_map(|i| {
                nums.windows(i as usize)
                    .map(|arr| arr.iter().sum::<i32>())
                    .filter(|&n| n > 0)
                    .min()
            })
            .min()
            .unwrap_or(-1)
    }
}
