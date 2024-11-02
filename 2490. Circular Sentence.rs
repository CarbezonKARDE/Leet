impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let chars: Vec<char> = sentence.chars().collect();
        if chars.first() != chars.last() {
            return false;
        }
        chars.windows(3)
            .filter(|w| w[1] == ' ')
            .all(|w| w[0] == w[2])
    }
}
