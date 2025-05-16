impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let n = words.len();
        let mut dp = vec![1; n];
        let mut prev = vec![None; n];
        let mut max = 1;
        let mut start = 0;
        for right in 1..n {
            for left in 0..right {
                if groups[left] != groups[right] && check(&words[left], &words[right]) {
                    if 1 + dp[left] > dp[right] {
                        dp[right] = 1 + dp[left];
                        prev[right] = Some(left);
                        if dp[right] > max {
                            max = dp[right];
                            start = right;
                        }
                    }
                }
            }
        }
        let mut res = vec![words[start].to_string()];
        while let Some(v) = prev[start] {
            res.push(words[v].to_string());
            start = v;
        }
        res.reverse();
        res
    }
}
fn check<T: AsRef<[u8]>>(a: T, b: T) -> bool {
    let (a, b) = (a.as_ref(), b.as_ref());
    a.len() == b.len() && a.iter().zip(b).filter(|&(&x, &y)| x != y).count() == 1
}
