impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut stack: Vec<Vec<i32>> = Vec::new();
        stack.push(intervals[0].clone());
        for i in 1..intervals.len() {
            let a = intervals[i][0];
            let b = intervals[i][1];
            let last = stack.last().unwrap();
            let x = last[0];
            let y = last[1];
            if x <= a && b <= y {
                continue;
            }
            stack.push(intervals[i].clone());
        }
        stack.len() as i32
    }
}
