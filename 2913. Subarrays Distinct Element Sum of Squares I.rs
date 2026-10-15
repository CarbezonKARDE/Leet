impl Solution {
  pub fn sum_counts(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for start in 0..nums.len() {
      let mut mask = 0u128;
      let mut count = 0i32;
      for &num in &nums[start..] {
        let bit = 1u128 << num;
        count += ((mask & bit) == 0) as i32;
        mask |= bit;
        sum += count * count;
      }
    }
    sum
  }
}
