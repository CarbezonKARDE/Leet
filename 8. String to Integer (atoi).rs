impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();
        let (s, sign) = match s.strip_prefix('-') {
            Some(s) => (s, -1),
            None => (s.strip_prefix('+').unwrap_or(s), 1),
        };
        s.chars()
            .map(|c| c.to_digit(10))
            .take_while(Option::is_some)
            .flatten()
            .fold(0, |acc, digit| {
                acc.saturating_mul(10).saturating_add(sign * digit as i32)
            })
    }
}
