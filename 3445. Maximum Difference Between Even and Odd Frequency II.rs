impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
                let n = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let k = k as usize;
        let mut ans = i32::MIN;
        for a in '0'..='4' {
            for b in '0'..='4' {
                if a == b {
                    continue;
                }
                let mut best = vec![i32::MAX; 4];
                let mut cnt_a = 0;
                let mut cnt_b = 0;
                let mut prev_a = 0;
                let mut prev_b = 0;
                let mut left: isize = -1;
                for right in 0..n {
                    if s[right] == a {
                        cnt_a += 1;
                    }
                    if s[right] == b {
                        cnt_b += 1;
                    }
                    while (right as isize - left) as usize >= k && cnt_b - prev_b >= 2 {
                        let left_status = Self::get_status(prev_a, prev_b);
                        best[left_status] = best[left_status].min(prev_a - prev_b);
                        left += 1;
                        if s[left as usize] == a {
                            prev_a += 1;
                        }
                        if s[left as usize] == b {
                            prev_b += 1;
                        }
                    }
                    let right_status = Self::get_status(cnt_a, cnt_b);
                    let required_status = right_status ^ 0b10;
                    if best[required_status] != i32::MAX {
                        ans = ans.max(cnt_a - cnt_b - best[required_status]);
                    }
                }
            }
        }
        ans
    }
    fn get_status(cnt_a: i32, cnt_b: i32) -> usize {
        (((cnt_a & 1) << 1) | (cnt_b & 1)) as usize
    }
}
