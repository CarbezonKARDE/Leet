impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = s.len();
        let n = p.len();
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        let is_match = |i: usize, j: usize| -> bool {
            j > 0 && (p.chars().nth(j - 1).unwrap() == '.' || s.chars().nth(i - 1).unwrap() == p.chars().nth(j - 1).unwrap())
        };
        for j in 0..p.len() {
            if p.chars().nth(j).unwrap() == '*' && dp[0][j - 1] {
                dp[0][j + 1] = true;
            }
        }
        for i in 0..m {
            for j in 0..n {
                if p.chars().nth(j).unwrap() == '*' {
                    let no_repeat = dp[i + 1][j - 1];
                    let do_repeat = is_match(i + 1, j) && dp[i][j + 1];
                    dp[i + 1][j + 1] = no_repeat || do_repeat;
                } else if is_match(i + 1, j + 1) {
                    dp[i + 1][j + 1] = dp[i][j];
                }
            }
        }
        dp[m][n]
    }
}
