impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        if n <= 2 {
            return n;
        }
        1 << (32 - n.leading_zeros())
    }
}
