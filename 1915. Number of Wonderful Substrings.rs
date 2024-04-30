use std::collections::HashMap;
impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut ans = 0;
        let mut prefix = 0i32;
        let mut count = HashMap::with_capacity(1024);
        count.insert(0, 1);
        for c in word.chars() {
            let c_index = c as i32 - 'a' as i32;
            prefix ^= 1 << c_index;
            ans += *count.get(&prefix).unwrap_or(&0);
            for i in 0..10 {
                ans += *count.get(&(prefix ^ 1 << i)).unwrap_or(&0);
            }
            *count.entry(prefix).or_insert(0) += 1;
        }
        ans
    }
}
