impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        for t in 0..=60 {
            let s: i128 = num1 as i128 - (t as i128) * (num2 as i128);
            if s < 0 { continue; }
            if s < t as i128 { continue; }
            let s_u = s as u128;
            let ones = (s_u as u64).count_ones() as u32
                     + ((s_u >> 64) as u64).count_ones() as u32;

            if (ones as i32) <= t as i32 {
                return t as i32;
            }
        }
        -1
    }
}
