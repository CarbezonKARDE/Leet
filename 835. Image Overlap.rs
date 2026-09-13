use std::collections::HashMap;
use std::cmp::max;
impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let mut ones1 = Vec::new();
        let mut ones2 = Vec::new();
        for r in 0..n {
            for c in 0..n {
                if img1[r][c] == 1 { ones1.push((r as i32, c as i32)); }
                if img2[r][c] == 1 { ones2.push((r as i32, c as i32)); }
            }
        }
        let mut count = HashMap::new();
        let mut max_overlap = 0;
        for &(r1, c1) in &ones1 {
            for &(r2, c2) in &ones2 {
                let key = (r1 - r2 + 100) * 1000 + (c1 - c2 + 100);
                let val = count.entry(key).or_insert(0);
                *val += 1;
                max_overlap = max(max_overlap, *val);
            }
        }
        max_overlap
    }
}
