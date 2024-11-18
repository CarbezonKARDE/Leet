use std::cmp::Ordering;
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![0; code.len()];
        }
        let (len,k) = (code.len() as isize, k as isize);
        (0..len)
            .map(|index| {
                match k.cmp(&0) {
                    Ordering::Less => (index+k..index),
                    Ordering::Greater => (index+1..index+k+1),
                    _ => unreachable!(),
                }
                .map(|offset| code[(offset.rem_euclid(len)) as usize])
                .sum()
            })
            .collect()
    }
}
