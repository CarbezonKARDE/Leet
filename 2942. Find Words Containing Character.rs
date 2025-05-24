impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
        .iter()
        .enumerate()
        .filter_map(|(idx, word)| word.contains(x).then(||idx as i32))
        .collect()
    }
}
