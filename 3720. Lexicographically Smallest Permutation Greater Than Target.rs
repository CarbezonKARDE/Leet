impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut cnt = [0i32; 26];
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }
        for b in target.bytes() {
            cnt[(b - b'a') as usize] -= 1;
        }
        for i in (0..target.len()).rev() {
            let cur = (target.as_bytes()[i] - b'a') as usize;
            cnt[cur] += 1;
            if cnt.iter().any(|&x| x < 0) {
                continue;
            }
            let mut next = None;
            for c in cur + 1..26 {
                if cnt[c] > 0 {
                    next = Some(c);
                    break;
                }
            }
            let Some(next) = next else {
                continue;
            };
            cnt[next] -= 1;
            let mut ans = target[..i].to_string();
            ans.push((b'a' + next as u8) as char);
            for c in 0..26 {
                for _ in 0..cnt[c] {
                    ans.push((b'a' + c as u8) as char);
                }
            }
            return ans;
        }
        String::new()
    }
}
