const MOD: i64 = 1_000_000_007;
const MX: usize = 100000;
static mut FACT: [i64; MX] = [0; MX];
static mut INV_FACT: [i64; MX] = [0; MX];
fn qpow(mut x: i64, mut n: i32) -> i64 {
    let mut res = 1;
    while n > 0 {
        if n & 1 == 1 {
            res = res * x % MOD;
        }
        x = x * x % MOD;
        n >>= 1;
    }
    res
}
fn init() {
    unsafe {
        if FACT[0] != 0 { return; }
        FACT[0] = 1;
        for i in 1..MX {
            FACT[i] = FACT[i - 1] * (i as i64) % MOD;
        }
        INV_FACT[MX - 1] = qpow(FACT[MX - 1], MOD as i32 - 2);
        for i in (1..MX).rev() {
            INV_FACT[i - 1] = INV_FACT[i] * (i as i64) % MOD;
        }
    }
}
fn comb(n: usize, m: usize) -> i64 {
    unsafe {
        FACT[n] * INV_FACT[m] % MOD * INV_FACT[n - m] % MOD
    }
}
impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        init();
        (comb((n - 1)as usize, k as usize) * m as i64 % MOD * qpow((m - 1) as i64, (n - k - 1) as i32) % MOD) as i32
    }
}
