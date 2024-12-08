impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let n = events.len();
        let mut events = events;
        events.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut suffix_max = vec![0; n];
        suffix_max[n - 1] = events[n - 1][2];
        for i in (0..n - 1).rev() {
            suffix_max[i] = suffix_max[i + 1].max(events[i][2]);
        }
        let mut max_sum = 0;
        for i in 0..n {
            let mut left = i + 1;
            let mut right = n - 1;
            let mut next_event_index = -1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if events[mid][0] > events[i][1] {
                    next_event_index = mid as i32;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            if next_event_index != -1 {
                let next_event_index = next_event_index as usize;
                max_sum = max_sum.max(events[i][2] + suffix_max[next_event_index]);
            }
            max_sum = max_sum.max(events[i][2]);
        }
        max_sum
    }
}
