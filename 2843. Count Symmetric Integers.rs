impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut res = 0;
        for a in low..=high {
            if a < 100 && a % 11 == 0 {
                res += 1;
            } else if 1000 <= a && a < 10000 {
                let left = a / 1000 + (a % 1000) / 100;
                let right = (a % 100) / 10 + a % 10;
                if left == right {
                    res += 1;
                }
            }
        }
        res
    }
}
