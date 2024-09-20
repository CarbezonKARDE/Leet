use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut result = Vec::new();
        if words.is_empty() || s.len() < words[0].len() * words.len() {
            return result;
        }
        let word_len = words[0].len();
        let all_words_len = word_len * words.len();
        let mut word_count = HashMap::new();
        for word in &words {
            *word_count.entry(word.clone()).or_insert(0) += 1;
        }
        for i in 0..word_len {
            let mut left = i;
            let mut right = i;
            let mut current_count = HashMap::new();
            let mut words_used = 0;
            while right + word_len <= s.len() {
                let word = &s[right..right + word_len];
                right += word_len;
                if let Some(&count) = word_count.get(word) {
                    *current_count.entry(word.to_string()).or_insert(0) += 1;
                    if current_count[word] <= count {
                        words_used += 1;
                    }
                    while words_used == words.len() {
                        if right - left == all_words_len {
                            result.push(left as i32);
                        }
                        
                        let left_word = &s[left..left + word_len];
                        if let Some(&count) = word_count.get(left_word) {
                            current_count.entry(left_word.to_string()).and_modify(|e| *e -= 1);
                            if current_count[left_word] < count {
                                words_used -= 1;
                            }
                        }
                        left += word_len;
                    }
                } else {
                    current_count.clear();
                    words_used = 0;
                    left = right;
                }
            }
        } 
        result
    }
}

