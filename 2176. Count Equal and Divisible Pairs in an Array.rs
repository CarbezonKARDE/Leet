impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
       let mut pair:i32=0;
       let n = nums.len();
    for i in 0..n - 1 {
        for j in i + 1..n {
            if nums[i] == nums[j] && (i * j) % (k as usize) == 0 {
                pair += 1;
            }
        }
    }
    pair
    }
}
