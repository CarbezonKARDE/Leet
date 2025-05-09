impl Solution {
    const MOD: i64 = 1e9 as i64 + 7;
    const FACTORIAL: [i64; 82] = {
        let mut factorial = [1; 82];
        let mut i = 2;
        while i <= 81 {
            factorial[i] = (i as i64 * factorial[i - 1]) % Self::MOD;
            i += 1;
        }
        factorial
    };
    const INV_FACTORIAL: [i64; 82] = {
        let factorial = Self::FACTORIAL;
        let mut inv_factorial = [1; 82];
        let mut i = 2;
        while i <= 81 {
            inv_factorial[i] = Self::mod_exp(factorial[i], Self::MOD - 2);
            i += 1;
        }
        inv_factorial
    };
    const fn mod_exp(mut base: i64, mut exp: i64) -> i64 {
        base %= Self::MOD;
        let mut result = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % Self::MOD;
            }
            exp >>= 1;
            base = (base * base) % Self::MOD;
        }
        result
    }
    pub fn count_balanced_permutations(num: String) -> i32 {
        let n = num.len();
        let (freq, total_sum) = num.bytes().fold(([0; 10], 0), |(mut freq, sum), b| {
            freq[(b - b'0') as usize] += 1;
            (freq, sum + (b - b'0') as usize)
        });
        if total_sum & 1 == 1 {
            return 0;
        }
        let target_sum = total_sum / 2;
        let target_len = n / 2;
        let ways = (Self::FACTORIAL[target_len] * Self::FACTORIAL[n - target_len]) % Self::MOD;
        let mut dp: Vec<Vec<Vec<i64>>> =
            vec![vec![vec![-1i64; target_sum + 1]; target_len + 1]; 10];
        Self::go(0, 0, 0, target_len, target_sum, ways, &freq, &mut dp) as i32
    }
    fn go(
        i: usize,
        len1: usize,
        sum1: usize,
        target_len: usize,
        target_sum: usize,
        ways: i64,
        freq: &[usize; 10],
        dp: &mut [Vec<Vec<i64>>],
    ) -> i64 {
        if i >= 10 {
            if len1 == target_len && sum1 == target_sum {
                return ways;
            }
            return 0;
        }
        if len1 > target_len || sum1 > target_sum {
            return 0;
        }
        if dp[i][len1][sum1] >= 0 {
            return dp[i][len1][sum1];
        }
        let mut res = 0;
        for take in 0..=freq[i] {
            let w = Self::go(
                i + 1,
                len1 + take,
                sum1 + take * i,
                target_len,
                target_sum,
                ways,
                freq,
                dp,
            );
            let w = (w * Self::INV_FACTORIAL[take]) % Self::MOD;
            let w = (w * Self::INV_FACTORIAL[freq[i] - take]) % Self::MOD;
            res = (res + w) % Self::MOD;
        }
        dp[i][len1][sum1] = res;
        res
    }
}
