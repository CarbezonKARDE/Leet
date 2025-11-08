impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut result = 0;
        let mut i = 0;
        while (1 << i) <= n {
            if n & (1 << i) != 0 {
                result = (1 << (i + 1)) - 1 - result;
            }
            i += 1;
        }
        result
    }
}
