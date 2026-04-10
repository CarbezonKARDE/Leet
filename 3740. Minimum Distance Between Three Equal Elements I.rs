use itertools::Itertools;
impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .map(|(i,v)| (v,i))
            .collect::<Vec<_>>()
            .iter()
            .sorted()
            .collect::<Vec<_>>()
            .windows(3)
            .filter(|w| w[0].0 == w[2].0)
            .map(|w| (w[1].1-w[0].1 + w[2].1-w[1].1 + w[2].1-w[0].1) as i32)
            .min()
            .unwrap_or(-1)
    }
}
