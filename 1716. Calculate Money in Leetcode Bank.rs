impl Solution {
    pub fn total_money(mut n: i32) -> i32 {
        let mut start = 1;
        let mut total = 0;
        while n > 0 {
            for day in start..start + 7 {
                if n <= 0 { break; }
                total += day;
                n -= 1;
            }
            start += 1;
        }
        total
    }
}
