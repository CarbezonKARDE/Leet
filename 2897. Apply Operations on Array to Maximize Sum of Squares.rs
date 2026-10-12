const MAX_VAL : i64   = 10_i64.pow(9);
const N_BITS  : usize = MAX_VAL.ilog2() as usize + 1;
const MOD     : i64   = 10_i64.pow(9) + 7;
impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut bit_counts = [0; N_BITS];
        let mut answer     = 0;
        for value in nums {
            let mut mask = 1;   
            for count in &mut bit_counts {
                if value & mask != 0 {
                    *count += 1;
                }
                mask <<= 1;
            }
        }
        for _ in 0..k {
            let mut value = 0;
            let mut mask  = 1;

            for count in &mut bit_counts {
                if *count > 0 {
                    value |= mask;
                    *count -= 1;
                }
                mask <<= 1;
            }
            answer = (answer + value * value) % MOD;
        }
        return answer as i32;
    }
}
