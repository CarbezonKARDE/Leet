impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        const PRIME_MASK: u32 = 0b10100010100010101100;
        (left..=right)
            .filter(|&i| PRIME_MASK & (1 << (i as u32).count_ones()) != 0)
            .count() as i32        
    }
}
