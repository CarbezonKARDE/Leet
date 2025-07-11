use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable();
        let mut counts = vec![0; n as usize];
        let mut available_rooms: BinaryHeap<_> = (0..n).map(Reverse).collect();
        let mut unavailable_rooms: BinaryHeap<Reverse<(usize, i32)>> = BinaryHeap::new();
        for meeting in meetings {
            while let Some(&Reverse((end_time, room))) = unavailable_rooms.peek() {
                if end_time > meeting[0] as usize {
                    break;
                }
                unavailable_rooms.pop();
                available_rooms.push(Reverse(room));
            }
            if let Some(Reverse(room)) = available_rooms.pop() {
                unavailable_rooms.push(Reverse((meeting[1] as usize, room)));
                counts[room as usize] += 1;
            } else {
                let Reverse((end_time, room)) = unavailable_rooms.pop().unwrap();
                unavailable_rooms.push(Reverse((end_time + (meeting[1] - meeting[0]) as usize, room)));
                counts[room as usize] += 1;
            }
        }
        counts.into_iter().enumerate().rev().max_by_key(|(_, c)| *c).unwrap().0 as _
    }
}
