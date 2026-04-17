use std::collections::HashMap;
impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
      #[inline(always)]
      fn mirror_value(mut x: i32) -> i32 {
        let mut rev = 0;
        while x > 0 {
          rev = 10*rev + x % 10;
          x /= 10;
        }
        rev
      }
      let mut last:HashMap<i32,usize> = HashMap::with_capacity(nums.len());
      last.insert(mirror_value(nums[0]), 0);
      match nums.iter().enumerate().skip(1).fold(None, |acc:Option<usize>, (i, &x)| {
        let min_dist = if let Some(prev) = last.get(&x) {
          match acc {
            Some(min_dist) => Some(min_dist.min(i - prev)),
            None => Some(i - prev)
          }
        } else {
          acc
        };
        last.insert(mirror_value(x), i);
        min_dist
      }) {
        Some(min_dist) => min_dist as i32,
        None => -1,
      }
    }
}
