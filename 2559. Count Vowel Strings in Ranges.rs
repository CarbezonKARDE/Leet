impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_sum = vec![0; words.len() + 1];
        for (i, word) in words.iter().enumerate() {
            prefix_sum[i + 1] = prefix_sum[i] + 
                if Self::is_vowel(word.as_bytes()[0]) && 
                   Self::is_vowel(word.as_bytes()[word.len()-1]) { 1 } else { 0 };
        }
        queries.iter().map(|q| {
            prefix_sum[q[1] as usize + 1] - prefix_sum[q[0] as usize]
        }).collect()
    }
    #[inline(always)]
    fn is_vowel(c: u8) -> bool {
        matches!(c, b'a' | b'e' | b'i' | b'o' | b'u')
    }
}
