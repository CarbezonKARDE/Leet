impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable_by_key(|meeting| meeting[0]);
        let mut start = days;
        let mut end = 0;
        for meeting in &meetings {
            start = start.min(meeting[0]);
            end = end.max(meeting[1]);
        }
        let mut free_days = (start - 1) + (days - end);
        let mut last = meetings[0][1];
        for meeting in meetings.iter().skip(1) {
            if meeting[0] > last {
                free_days += meeting[0] - last - 1;
            }
            last = last.max(meeting[1]);
        }
        free_days
    }
}
