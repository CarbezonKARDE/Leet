impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut prefix = vec![vec![0; n + 1]; m + 1];
        for i in 0..m {
            for j in 0..n {
                prefix[i + 1][j + 1] = mat[i][j] + prefix[i][j + 1] + prefix[i + 1][j] - prefix[i][j];
            }
        }
        let mut left = 0;
        let mut right = m.min(n);
        let mut ans = 0;
        while left <= right {
            let mid = (left + right) / 2;
            let mut found = false;
            'outer: for i in mid..=m {
                for j in mid..=n {
                    let total = prefix[i][j] - prefix[i - mid][j] - prefix[i][j - mid] + prefix[i - mid][j - mid];
                    if total <= threshold {
                        found = true;
                        break 'outer;
                    }
                }
            }
            if found {
                ans = mid as i32;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}
