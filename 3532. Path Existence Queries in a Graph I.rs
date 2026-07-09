impl Solution {
    pub fn path_existence_queries(mut n: i32, mut a: Vec<i32>, m: i32, q: Vec<Vec<i32>>) -> Vec<bool> {
        let mut p = a[0];
        for x in &mut a {
            n += (*x - p > m) as i32;
            p = *x;
            *x = n
        }
        q.iter().map(|z| a[z[0] as usize] == a[z[1] as usize]).collect()
    }
}
