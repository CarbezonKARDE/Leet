impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut letter_freq = vec![0; 26];
        for c in word.chars() {
            letter_freq[(c as u8 - b'a') as usize] += 1;
        }
        
        letter_freq.sort_unstable_by(|a, b| b.cmp(a));
        
        let mut total_presses = 0;
        for (i, &freq) in letter_freq.iter().enumerate() {
            if freq == 0 {
                break;
            }
            total_presses += (i / 8 + 1) as i32 * freq;
        }
        
        total_presses
    }
}
