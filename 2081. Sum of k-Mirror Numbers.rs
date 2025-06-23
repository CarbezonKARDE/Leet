impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        fn get_next_palindrome(previous: &mut Vec<i64>, base: i64) {
            let length = previous.len();
            let middle = length >> 1;
            for i in middle..length {
                if previous[i] + 1 < base {
                    previous[i] = previous[i] + 1;
                    previous[length - i - 1] = previous[i];
                    for j in middle..i {
                        previous[j] = 0;
                        previous[length - j - 1] = 0;
                    }
                    return;
                }
            }
            let mut current = vec![0; length];
            current[0] = 1;
            current.push(1);
            std::mem::swap(&mut current, previous);
        }
        pub fn is_palindrome(x: i64) -> bool {
            if x < 0 {
                return false;
            }
            let mut z = x;
            let mut y = 0;
            while z > 0 {
                y = y * 10 + z % 10;
                z = z / 10
            }
            return x == y;
        }
        let mut current = vec![0];
        let mut result = 0;
        let k = k as i64;
        for _ in 0..n {
            loop {
                get_next_palindrome(&mut current, k);
                let base10 = current
                    .iter()
                    .fold((0, 1), |(n, s), &d| (n + s * d, s * k))
                    .0;
                if is_palindrome(base10) {
                    result += base10;
                    break;
                }
            }
        }
        result
    }
}
