impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = vec![];
        let mut value: u64 = 0;
        for i in nums {
            value = ((value << 1) | i as u64) % 5;
            result.push(value == 0);
        }
        result
    }
}
