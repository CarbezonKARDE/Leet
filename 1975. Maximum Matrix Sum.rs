use std::f64::INFINITY;
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut sum = 0 as i64;
        let mut min = INFINITY as i64;
        let mut neg_count = 0;
        for row in matrix {
            for elem in row {
                sum += elem.abs() as i64;
                if elem < 0 { neg_count += 1 }
                min = *[elem.abs() as i64, min].iter().min().unwrap();
            }
        }
        sum + {if neg_count%2 == 0 { 0 as i64 } else { -2 * min }}
    }
}
