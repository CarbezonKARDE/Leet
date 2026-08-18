impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter()
            .filter(|&n| {
                k == 1 || k == nums.len() as i32 || *n == nums[0] || n == nums.last().unwrap()
            })
            .fold([0i32; 51], |mut f, &n| {
                f[n as usize] += 1;
                f
            })
            .into_iter()
            .zip(0..)
            .filter_map(|(f, n)| (f > 0 && (f == 1 || k == nums.len() as i32)).then_some(n))
            .max()
            .unwrap_or(-1)
    }
}
