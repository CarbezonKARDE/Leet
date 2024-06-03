impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut i = 0;
        let t_chars: Vec<char> = t.chars().collect();
        for c in s.chars() {
            if i < t_chars.len() && c == t_chars[i] {
                i += 1;
                if i == t_chars.len() {
                    return 0;
                }
            }
        }
        (t.len() - i) as i32
    }
}
