impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let mut coins = coins;
        coins.sort();
        if coins[0] == 1 {
            return k as i64;
        }
        let mut n = coins.len();
        let mut r = 0;
        let mut i = 0;
        while i < n - r - 1 {
            let c = coins[i];
            let mut j = i + 1;
            while j < n - r {
                if coins[j] % c == 0 {
                    coins.remove(j);
                    r += 1;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
        let a = n - r;
        if a == 1 {
            return (coins[0] as i64) * (k as i64);
        }
        coins.truncate(a);
        let gcd = |mut a: i64, mut b: i64| {
            while b > 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        };
        let lcm = |a: i64, b: i64| a / gcd(a, b) * b;
        let count_nums = |m: i64| -> i64 {
            let mut sum = 0i64;
            let total_masks = 1 << a;
            for mask in 1..total_masks {
                let mut common = 1i64;
                let mut bits = 0;
                for i in 0..a {
                    if (mask & (1 << i)) != 0 {
                        common = lcm(common, coins[i] as i64);
                        bits += 1;
                    }
                }
                if (bits & 1) == 1 {
                    sum += m / common;
                } else {
                    sum -= m / common;
                }
            }
            sum
        };
        let mut low = coins[0] as i64;
        let mut high = (coins[0] as i64) * (k as i64);
        let mut result = 0i64;
        while low <= high {
            let mid = low + (high - low) / 2;
            let count = count_nums(mid);
            if count >= k as i64 {
                result = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        result
    }
}
