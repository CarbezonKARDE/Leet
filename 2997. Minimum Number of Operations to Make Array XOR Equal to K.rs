impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let xors: i32 = nums.iter().fold(0, |acc, &x| acc ^ x);
        (k ^ xors).count_ones() as i32
    }
}
