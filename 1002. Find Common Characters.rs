impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();
        let mut common_count = vec![i32::MAX; 26];
        for word in &words {
            let mut count = vec![0; 26];
            for c in word.chars() {
                count[(c as usize) - ('a' as usize)] += 1;
            }
            for i in 0..26 {
                common_count[i] = common_count[i].min(count[i]);
            }
        }
        for c in 'a'..='z' {
            for _ in 0..common_count[(c as usize) - ('a' as usize)] {
                ans.push(c.to_string());
            }
        }
        ans
    }
}
