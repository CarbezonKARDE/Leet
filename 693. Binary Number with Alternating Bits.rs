impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut start: i64 = 1;
        while start < n as i64 {
            if (start % 2 == 0) {
                start = start * 2 + 1
            } else {
                start = start * 2
            }
        }
        start == n as i64
    }
}
