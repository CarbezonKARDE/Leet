impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let mut s = s.into_bytes();
        let n = s.len();
        let mut half = s[..n / 2].to_vec();
        half.sort_unstable();
        let mut k = k as i64 - 1;
        let mut rearrange = [0; 26];
        let mut count = 0;
        let mut perm = 1;
        while perm <= k {
            let ch_idx = match half.pop() {
                Some(ch) => (ch - b'a') as usize,
                None => return String::new(),
            };
            count += 1;
            rearrange[ch_idx] += 1;
            perm *= count;
            perm /= rearrange[ch_idx];
        }
        while count > 0 {
            for ch_idx in 0..26 {
                if rearrange[ch_idx] == 0 {
                    continue;
                }
                let mut next_perm = perm;
                next_perm *= rearrange[ch_idx];
                next_perm /= count;
                if k < next_perm {
                    rearrange[ch_idx] -= 1;
                    count -= 1;
                    half.push(ch_idx as u8 + b'a');
                    perm = next_perm;
                    break;
                }
                k -= next_perm;
            }
        }
        s[..n / 2].copy_from_slice(&half);
        half.reverse();
        s[n.div_ceil(2)..].copy_from_slice(&half);
        String::from_utf8(s).unwrap()
    }
}
