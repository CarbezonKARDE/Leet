impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let n = points.len();
        let mut result = 0;
        for i in 0..n {
            let top = points[i][1];
            let mut bot = i32::MIN;
            for j in (i + 1)..n {
                let y = points[j][1];
                if bot < y && y <= top {
                    result += 1;
                    bot = y;
                    if bot == top {
                        break;
                    }
                }
            }
        }
        result
    }
}
