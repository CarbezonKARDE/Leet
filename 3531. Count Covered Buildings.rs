impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut max_x = vec![0usize; n+1];
        let mut min_x = vec![usize::MAX; n+1];
        let mut max_y = vec![0usize; n+1];
        let mut min_y = vec![usize::MAX; n+1];
        for b in &buildings {
            let x = b[0] as usize;
            let y = b[1] as usize;
            max_x[x] = max_x[x].max(y);
            min_x[x] = min_x[x].min(y);
            max_y[y] = max_y[y].max(x);
            min_y[y] = min_y[y].min(x);
        }
        let mut ans = 0;
        for b in &buildings {
            let x = b[0] as usize;
            let y = b[1] as usize;
            if y > min_x[x] && y < max_x[x] && x > min_y[y] && x < max_y[y] {
                ans += 1;
            }
        }
        ans
    }
}
