impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut last_lower = vec![-1; 26];
        let mut first_upper = vec![-1; 26];
        let bytes = word.as_bytes();
        for (i, &b) in bytes.iter().enumerate() {
            if b >= b'a' && b <= b'z' {
                let idx = (b - b'a') as usize;
                last_lower[idx] = i as i32;
            } else if b >= b'A' && b <= b'Z' {
                let idx = (b - b'A') as usize;
                if first_upper[idx] == -1 {
                    first_upper[idx] = i as i32;
                }
            }
        }
        let mut count = 0;
        for i in 0..26 {
            if last_lower[i] != -1 && first_upper[i] != -1 {
                if last_lower[i] < first_upper[i] {
                    count += 1;
                }
            }
        }
        count
    }
}
