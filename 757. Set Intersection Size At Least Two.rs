impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| {
            if a[1] == b[1] {
                b[0].cmp(&a[0])
            } else {
                a[1].cmp(&b[1])
            }
        });
        let mut result = 0;
        let mut last = -1;
        let mut second_last = -1;
        for interval in intervals {
            let a = interval[0];
            let b = interval[1];
            if a > last {
                result += 2;
                last = b;
                second_last = b - 1;
            } else if a > second_last {
                result += 1;
                second_last = last;
                last = b;
            }
        }
        result
    }
}
