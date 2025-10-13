impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = Vec::with_capacity(words.len());
        let mut prev_sig: [u8; 26] = [0; 26];
        let mut has_prev = false;
        for w in words.into_iter() {
            let mut sig = [0u8; 26];
            for &b in w.as_bytes() {
                let idx = (b - b'a') as usize;
                sig[idx] += 1;
            }
            if has_prev && sig == prev_sig {
                continue;
            }
            prev_sig = sig;
            has_prev = true;
            res.push(w);
        }
        res
    }
}
