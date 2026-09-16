impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n_val = (n + k - 1) as i64;
        let k_val = (2 * k) as i64;
        if k_val > n_val {
            return 0;
        }
        let mut num = 1i64;
        let mut den = 1i64;
        for i in 1..=k_val {
            num = (num * (n_val - i + 1)) % MOD;
            den = (den * i) % MOD;
        }
        let power = |mut base: i64, mut exp: i64| -> i64 {
            let mut res = 1i64;
            base %= MOD;
            while exp > 0 {
                if exp % 2 == 1 {
                    res = (res * base) % MOD;
                }
                base = (base * base) % MOD;
                exp /= 2;
            }
            res
        };
        ((num * power(den, MOD - 2)) % MOD) as i32
    }
}
