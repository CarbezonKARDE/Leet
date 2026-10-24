impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        fn day_of_year(date: &str) -> i32 {
            const DAYS_BEFORE_MONTH: [i32; 12] = [
                0, 31, 59, 90, 120, 151,
                181, 212, 243, 273, 304, 334,
            ];
            let bytes = date.as_bytes();
            let month =
                (bytes[0] - b'0') as usize * 10 + (bytes[1] - b'0') as usize;
            let day =
                (bytes[3] - b'0') as i32 * 10 + (bytes[4] - b'0') as i32;
            DAYS_BEFORE_MONTH[month - 1] + day
        }
        let start = day_of_year(&arrive_alice)
            .max(day_of_year(&arrive_bob));
        let end = day_of_year(&leave_alice)
            .min(day_of_year(&leave_bob));
        (end - start + 1).max(0)
    }
}
