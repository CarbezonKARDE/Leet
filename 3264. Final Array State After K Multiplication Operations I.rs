impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        (0..k)
        .fold(nums, |mut nums, _| {
            nums
            .iter()
            .min()
            .copied()
            .and_then(|min_num| {
                nums
                .iter_mut()
                .find(move |num| **num == min_num)
            })
            .map(move |num| *num *= multiplier);
            nums
        })
    }
}
