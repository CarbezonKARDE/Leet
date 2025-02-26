impl Solution {
  pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let (mut max, mut cur, mut ncur) = (0, 0, 0);
    for num in nums {
      cur += num;
      ncur -= num;
      if cur < 0 {
        cur = 0;
      }
      if ncur < 0 {
        ncur = 0;
      }
      if cur > max {
        max = cur;
      }
      if ncur > max {
        max = ncur;
      }
    }
    max
  }
}
