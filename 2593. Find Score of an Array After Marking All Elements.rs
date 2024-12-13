impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut nums = nums
          .into_iter()
          .enumerate()
          .collect::<Vec<_>>();
        nums.sort_unstable_by_key(|&(i, n)| (n, i));
        let n = nums.len();
        let mut occur = vec![0_u128; n / 128 + 1];
        nums
          .into_iter()
          .fold(0_i64, |sum, (i, x)| {
            if (occur[i / 128] >> (i % 128)) & 1 == 1 {
                sum
            } else {
                if i > 0 {
                    occur[(i - 1) / 128] |= 1 << ((i - 1) % 128);
                }
                if i < n - 1 {
                    occur[(i + 1) / 128] |= 1 << ((i + 1) % 128);
                }
                sum + x as i64
            }
          })
    }
}
