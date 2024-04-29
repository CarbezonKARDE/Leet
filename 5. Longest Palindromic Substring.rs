impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }
        let mut indices = (0, 0);
        for i in 0..s.len() {
            let (l1, r1) = Self::extend(&s, i, i);
            if r1 - l1 > indices.1 - indices.0 {
                indices = (l1, r1);
            }
            if i + 1 < s.len() && s.chars().nth(i).unwrap() == s.chars().nth(i + 1).unwrap() {
                let (l2, r2) = Self::extend(&s, i, i + 1);
                if r2 - l2 > indices.1 - indices.0 {
                    indices = (l2, r2);
                }
            }
        }
        s[indices.0..=indices.1].to_string()
    }
    fn extend(s: &String, mut i: usize, mut j: usize) -> (usize, usize) {
        while i > 0 && j < s.len() - 1 && s.chars().nth(i - 1).unwrap() == s.chars().nth(j + 1).unwrap() {
            i -= 1;
            j += 1;
        }
        (i, j)
    }
}
