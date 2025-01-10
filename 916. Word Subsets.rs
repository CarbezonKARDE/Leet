use std::collections::HashMap;
impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut maxfreq = vec![0; 26];
        let mut ans = Vec::new();
        for str in words2 {
            let mut freq = vec![0; 26];
            for ch in str.chars() {
                freq[ch as usize - 'a' as usize] += 1;
            }
            for i in 0..26 {
                maxfreq[i] = maxfreq[i].max(freq[i]);
            }
        }
        for str in words1 {
            let mut freq = vec![0; 26];
            for ch in str.chars() {
                freq[ch as usize - 'a' as usize] += 1;
            }
            let issubset = (0..26).all(|i| freq[i] >= maxfreq[i]);
            if issubset {
                ans.push(str);
            }
        }
        ans
    }
}
