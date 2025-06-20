use std::cmp::{min, max};
impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let (mut latitude, mut longitude, mut ans) = (0 as i32, 0 as i32, 0 as i32);
        let n = s.len();
        for (i, c) in s.chars().enumerate() {
            match c {
                'N' => latitude += 1,
                'S' => latitude -= 1,
                'E' => longitude += 1,
                'W' => longitude -= 1,
                _ => (),
            }
            let current = min(
                latitude.abs() + longitude.abs() + k * 2,
                (i + 1) as i32
            );
            ans = max(ans, current);
        }
        ans
    }
}
