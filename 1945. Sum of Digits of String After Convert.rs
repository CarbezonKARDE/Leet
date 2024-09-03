impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut s = s.chars().map(|c| (c as u8 - b'a' + 1).to_string()).collect::<Vec<String>>().join(""); 
        for _ in 0..k {
            s = s.chars().map(|c| c as i32 - '0' as i32).sum::<i32>().to_string();
        }
        return s.parse::<i32>().unwrap();
    }
}
