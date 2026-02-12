impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut result = 0;
        for start in 0..n {
            let mut counts = [0u16; 26];
            let mut distinct = 0u16;
            for end in start..n {
                let idx = (bytes[end] - b'a') as usize;
                if counts[idx] == 0 {
                    distinct += 1;
                }
                counts[idx] += 1;
                let len = (end - start + 1) as u16;
                if len % distinct == 0 {
                    let expected = len / distinct;
                    if Self::check_freq(&counts, distinct, expected) {
                        result = result.max(len);
                    }
                }
            }
        }
        result as i32
    }
    #[inline(always)]
    fn check_freq(counts: &[u16; 26], distinct: u16, expected: u16) -> bool {
        let mut found = 0;
        for &count in counts {
            if count > 0 {
                if count != expected {
                    return false;
                }
                found += 1;
                if found == distinct {
                    return true;
                }
            }
        }
        false
    }
}
