impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        nums.into_iter()
            .zip(0..)
            .filter(|(num, _)| *num == target)
            .fold(i32::MAX, |min, (_, i)| min.min((i - start).abs()))
    }
}
