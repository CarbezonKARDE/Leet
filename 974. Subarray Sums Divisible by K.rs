impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut prefix = 0;
        let mut count = vec![0; k as usize];
        count[0] = 1;
        for &num in &nums {
            prefix = (prefix + num % k + k) % k;
            ans += count[prefix as usize];
            count[prefix as usize] += 1;
        }
        ans
    }
}
