impl Solution {
  pub fn distinct_subseq_ii(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut last = [0i64; 26];
    let mut total = 1i64;
    for ch in s.bytes() {
        let idx = (ch - b'a') as usize;
        let prev = last[idx];
        last[idx] = total;
        total = (2 * total - prev).rem_euclid(MOD);
    }
    ((total - 1).rem_euclid(MOD)) as i32
  }
}
