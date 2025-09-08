impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        fn is_no_zero(mut num: i32) -> bool {
            while num > 0 {
                if num % 10 == 0 { return false; }
                num /= 10;
            }
            true
        }
        for a in 1..n {
            let b = n - a;
            if is_no_zero(a) && is_no_zero(b) {
                return vec![a, b];
            }
        }
        vec![1, 1]
    }
}
