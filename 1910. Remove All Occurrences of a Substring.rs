impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut result = s.into_bytes();
        while let Some(p) = result.windows(part.len()).position(|win| win == part.as_bytes()) {
            result.drain(p..part.len() + p);
        }
        result.into_iter().map(|b| b as char).collect()
    }
}
