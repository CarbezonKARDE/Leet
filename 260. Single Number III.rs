use std::iter::Sum;
use std::ops::BitXor;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xors = nums.iter().fold(0, |acc, &x| acc.bitxor(x));
        let lowbit = xors & -xors;
        let mut ans = vec![0, 0];

        for &num in nums.iter() {
            if num & lowbit != 0 {
                ans[0] ^= num;
            } else {
                ans[1] ^= num;
            }
        }

        ans
    }
}
