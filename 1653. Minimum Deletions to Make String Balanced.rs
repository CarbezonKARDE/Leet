impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut curr = s.bytes().filter(|&byte| byte == b'a').count();
        let mut best = curr;

        for byte in s.bytes() {
            if byte == b'a' {
                curr -= 1;
            } else {
                curr += 1;
            }
            best = best.min(curr);
        }
        best as i32
    }
}
